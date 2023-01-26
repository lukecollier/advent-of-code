package dev.lukecollier

import cats.effect.{IOApp, IO}
import cats.syntax.all._

import cats.effect.Concurrent
import fs2.{hash, text}
import fs2.io.file.{Files, Path}
import cats.effect.ExitCode
import scala.util.Try
import cats.kernel.Eq
import cats.instances.boolean
import cats.kernel.Semigroup
import scala.concurrent.duration._
import cats.parse.Parser
import cats.parse.Numbers.{digits, bigInt}
import cats.parse.Parser.{char => pchar, anyChar, charIn}
import cats.parse.Rfc5234.{sp, cr, crlf}
import scala.collection.mutable.Stack
import cats.instances.char
import cats.data.NonEmptyList
import cats.parse.Rfc5234
import fs2.Chunk
import cats.Foldable

/** @link
  *   https://adventofcode.com/2022/day/7
  * @note really good fun! wouldn't mind turning this into a small terminal emulation or vfs for fun & no profit
  */
object Seven extends LineStringDay[BigInt, BigInt]() {
  val commandStart = '$'
  sealed trait Filesystem {
    def name: String
    def size: BigInt
    def fold[S](s: S)(f: (S, Filesystem) => S): S
  }
  final case class Directory(
      name: String,
      contents: List[Filesystem]
  ) extends Filesystem {
    // todo: This is foldable but a file isn't actually foldable
    def fold[S](s: S)(f: (S, Filesystem) => S): S =
      contents.foldLeft(s) { case (acc, filesystem) =>
        filesystem.fold(f(acc, filesystem))(f)
      }
    def directories: List[Directory] = contents.collect { case dir: Directory =>
      dir
    }
    def size: BigInt = contents.foldLeft(BigInt("0")) {
      case (acc, filesystem) =>
        acc + filesystem.size
    }
    def add(filesystem: Filesystem) =
      this.copy(contents = contents :+ filesystem)

    // todo: Add tests when this obviously doesn't work
    // also turn it into a for comp with the inner
    def update(path: Path, f: Directory => Directory): Option[Directory] = {
      def inner(current: Directory, names: Seq[String]): Option[Directory] = {
        names.headOption match {
          case None => Some(f(current)) // success! return mutated current
          case Some(name) =>
            current.contents.zipWithIndex
              .collect { case (dir: Directory, idx) =>
                (dir, idx)
              }
              .find { case (dir, _) => dir.name == name } match {
              case None => None // path not found :( short circuit!
              case Some((value, idx)) =>
                inner(
                  value,
                  names.drop(1)
                ) match {
                  case Some(value) =>
                    Some(
                      current.copy(contents =
                        current.contents.updated(idx, value)
                      )
                    )
                  case None => None
                }
            }
        }
      }

      inner(this, path.names.map(_.toString))
    }

  }
  final case class File(size: BigInt, name: String) extends Filesystem {
    def fold[S](s: S)(f: (S, Filesystem) => S): S = f(s, this)
  }

  object Filesystem {
    def root: Directory = Directory("/", List.empty)
    // we need a cwd that changes with CD
    // we need to capture the output over multiple lines
    // A string buffer could exist that captures lines until a parsing failure
    lazy val directoryP: Parser[Directory] =
      (Parser.string("dir") ~ sp *> anyChar.rep).map(name =>
        Directory(name.mkString_(""), List.empty)
      )
    lazy val fileP: Parser[File] =
      (bigInt, sp, anyChar.rep).tupled.map { case (size, _, name) =>
        File(size, name.mkString_(""))
      }
    lazy val filesystemP: Parser[Filesystem] = directoryP.orElse(fileP)
  }

  sealed trait Command
  case class ChangeDirectories(args: String) extends Command
  case class ListDirectories() extends Command

  object Command {
    val lsP: Parser[ListDirectories] =
      (Parser.string("$ ls")).map { _ =>
        ListDirectories()
      }
    val cdP: Parser[ChangeDirectories] =
      (Parser.string("$ cd ") *> anyChar.rep.string)
        .map { args =>
          ChangeDirectories(args)
        }
    val commandP: Parser[Command] =
      cdP | lsP
  }

  case class Terminal(
      cwd: Path,
      reading: Boolean,
      root: Directory
  ) {
    def execute: Command => Terminal = {
      case ChangeDirectories(args) =>
        changeDirectory(args)
      case ListDirectories() =>
        this.copy(reading = true)
    }

    // todo: could be better
    def addFilesystem(other: Filesystem) = {
      val updatedRoot = root.update(cwd, _.add(other))
      if (updatedRoot.isEmpty) {
        System.err.println(
          s"Couldn't find a file subsystem mentioned $updatedRoot"
        )
      }
      this.copy(root = updatedRoot.get)
    }

    def changeDirectory(args: String) = this.copy(cwd = cwd.resolve(args))
  }

  object Terminal {
    def default =
      Terminal(
        cwd = Path.apply("/"),
        reading = false,
        root = Filesystem.root
      )
  }
  override def resourceFileName = "seven.txt"
  override def puzzleOne = _.map(line => (line, line.startsWith("$")))
    .fold(Terminal.default) {
      case (terminal, (line, true)) if terminal.reading => {
        Command.commandP.parseAll(line) match {
          case Right(command) => terminal.execute(command).copy(reading = false)
          case Left(error) => {
            System.err.println(s"problem parsing '$line' with error: $error")
            terminal.copy(reading = false)
          }
        }
      }
      case (terminal, (line, false)) if terminal.reading =>
        Filesystem.filesystemP.parseAll(line) match {
          case Left(value) => {
            System.err.println(s"error parsing filesystem: $value")
            terminal
          }
          case Right(value) => terminal.addFilesystem(value)
        }
      case (terminal, (line, true)) =>
        Command.commandP.parseAll(line) match {
          case Right(command) => terminal.execute(command)
          case Left(error) => {
            System.err.println(s"problem parsing $line with error: $error")
            terminal
          }
        }
      case (terminal, (line, false)) =>
        println(
          s"WARNING: '$line' was found but we're not buffering and it's not a command!"
        )
        terminal
    }
    .compile
    .lastOrError
    .map(_.root.fold(BigInt("0")) {
      case (acc, dir: Directory) if dir.size < 100000 => {
        acc + dir.size
      }
      case (acc, filesystem) => {
        acc
      }
    })
  override def puzzleTwo = _.map(line => (line, line.startsWith("$")))
    .fold(Terminal.default) {
      case (terminal, (line, true)) if terminal.reading => {
        Command.commandP.parseAll(line) match {
          case Right(command) => terminal.execute(command).copy(reading = false)
          case Left(error) => {
            System.err.println(s"problem parsing '$line' with error: $error")
            terminal.copy(reading = false)
          }
        }
      }
      case (terminal, (line, false)) if terminal.reading =>
        Filesystem.filesystemP.parseAll(line) match {
          case Left(value) => {
            System.err.println(s"error parsing filesystem: $value")
            terminal
          }
          case Right(value) => terminal.addFilesystem(value)
        }
      case (terminal, (line, true)) =>
        Command.commandP.parseAll(line) match {
          case Right(command) => terminal.execute(command)
          case Left(error) => {
            System.err.println(s"problem parsing $line with error: $error")
            terminal
          }
        }
      case (terminal, (line, false)) =>
        println(
          s"WARNING: '$line' was found but we're not buffering and it's not a command!"
        )
        terminal
    }
    .compile
    .lastOrError
    .map(terminal => {
      val totalDiskSpace = 70000000
      val neededSpace = 30000000
      val currentSize = terminal.root.size
      val remainingSize = totalDiskSpace - currentSize
      val minimumDelete = neededSpace - remainingSize
      terminal.root.fold(currentSize) {
        case (acc, dir: Directory)
            if dir.size > minimumDelete && dir.size < acc =>
          dir.size
        case (acc, _) => acc
      }
    })
}
