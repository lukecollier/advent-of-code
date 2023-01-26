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
  */
object Six extends IOApp.Simple {
  val partOne: fs2.Stream[IO, Char] => IO[Option[Long]] =
    _.sliding(4).zipWithIndex
      .map { case (chunk, idx) =>
        (chunk, idx + chunk.size)
      }
      .find { case (sliding, idx) =>
        sliding.toList.size == sliding.toList.toSet.size
      }
      .compile
      .last
      .map(_._2F)

  val partTwo: fs2.Stream[IO, Char] => IO[Option[Long]] =
    _.sliding(14).zipWithIndex
      .map { case (chunk, idx) =>
        (chunk, idx + chunk.size)
      }
      .find { case (sliding, idx) =>
        sliding.toList.size == sliding.toList.toSet.size
      }
      .compile
      .last
      .map(_._2F)

  val run = (for {
    outputOne <- partOne(
      FileIO
        .getInputPerChar[IO]("six.txt")
    )
    outputTwo <- partTwo(
      FileIO
        .getInputPerChar[IO]("six.txt")
    )
    _ <- IO.println(s"Part1: $outputOne")
    _ <- IO.println(s"Part2: $outputTwo")
  } yield ())
}
