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

/** @link
  *   https://adventofcode.com/2022/day/3
  * @TODO
  *   this one need's CLEANSING
  */
object Five extends IOApp.Simple {
  case class Move(amount: BigInt, from: BigInt, to: BigInt)
  case class Crate(label: Char) {
    override def toString = "[" ++ label.toString ++ "]"
  }
  // todo: using map is probs not good
  case class Crates(inner: Map[Int, List[Crate]]) {
    def tops: List[Option[Crate]] =
      inner.mapValues(_.lastOption).toList.sortBy(_._1).map(_._2)
    def addRow(crates: List[Option[Crate]]) =
      Crates(inner |+| crates.zipWithIndex.collect { case (Some(value), key) =>
        key + 1 -> List(value)
      }.toMap)

    def reverseColumns = Crates(inner.map { case (idx, crates) =>
      idx -> crates.reverse
    })
    def emulateCrateMover9000(move: Move) = for {
      (remaining, taken) <- inner
        .get(move.from.toInt)
        .map(col => col.splitAt(col.length - move.amount.toInt))
      toCol <- inner.get(move.to.toInt).map(_ ++ taken.reverse)
    } yield Crates(
      inner
        .updated(move.from.toInt, remaining)
        .updated(move.to.toInt, toCol)
    )

    def emulateCrateMover9001(move: Move) = for {
      (remaining, taken) <- inner
        .get(move.from.toInt)
        .map(col => col.splitAt(col.length - move.amount.toInt))
      toCol <- inner.get(move.to.toInt).map(_ ++ taken)
    } yield Crates(
      inner
        .updated(move.from.toInt, remaining)
        .updated(move.to.toInt, toCol)
    )

  }

  object Crates {
    val empty = Crates(Map.empty)
    val emptyCratesP = sp.rep(3, 3).map(_ => None)
    val crateP = charIn('A' to 'Z')
      .between(pchar('['), pchar(']'))
      .map(Crate(_))
      .map(_.some)
    val cratesP = (crateP.orElse(emptyCratesP) <* sp.?).repUntil(crlf)

    def preword(word: String) = (Parser.string(word) ~ sp) *> bigInt <* sp.?
    val moveP =
      (preword("move"), preword("from"), preword("to")).tupled
        .map { case (index, origin, destination) =>
          Move(index, origin, destination)
        }

    val eitherP = Crates.cratesP.eitherOr(Crates.moveP)
  }

  type ParserState = (Crates, Boolean)

  val partOne: fs2.Stream[IO, String] => IO[Crates] =
    _.fold((Crates.empty, true)) {
      case ((crates, true), line) => {
        val rowEither = Crates.cratesP
          .parseAll(line)
          .map(_.toList)
        rowEither match {
          case Right(row) => (crates.addRow(row), true)
          case Left(_) =>
            (
              crates.reverseColumns,
              false
            ) // disable init mode / enable move reader mode
        }
      }
      case ((crates, false), line) =>
        val moveEither = Crates.moveP.parseAll(line)
        moveEither match {
          case Right(move) => {
            val newCrates = crates.emulateCrateMover9000(move).get
            (newCrates, false)
          }
          case Left(_) => (crates, false) // not a move line, forget about it!
        }
    }.compile.lastOrError.map(_._1)

  val partTwo: fs2.Stream[IO, String] => IO[Crates] =
    _.fold((Crates.empty, true)) {
      case ((crates, true), line) => {
        val rowEither = Crates.cratesP
          .parseAll(line)
          .map(_.toList)
        rowEither match {
          case Right(row) => (crates.addRow(row), true)
          case Left(_) =>
            (
              crates.reverseColumns,
              false
            ) // disable init mode / enable move reader mode
        }
      }
      case ((crates, false), line) =>
        val moveEither = Crates.moveP.parseAll(line)
        moveEither match {
          case Right(move) => {
            val newCrates = crates.emulateCrateMover9001(move).get
            (newCrates, false)
          }
          case Left(_) => (crates, false) // not a move line, forget about it!
        }
    }.compile.lastOrError.map(_._1)

  val run = (for {
    outputOne <- partOne(
      FileIO
        .getInput[IO]("five.txt")
    ).map(_.tops.collect { case Some(v) => v.label }.mkString(""))
    outputTwo <- partTwo(
      FileIO
        .getInput[IO]("five.txt")
    ).map(_.tops.collect { case Some(v) => v.label }.mkString(""))
    _ <- IO.println(s"Part1: $outputOne")
    _ <- IO.println(s"Part2: $outputTwo")
  } yield ())
}
