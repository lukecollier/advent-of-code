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
import cats.parse.Parser.{char => pchar}

/** @link
  *   https://adventofcode.com/2022/day/3
  */
object Four extends IOApp.Simple {
  case class Range(low: BigInt, high: BigInt) {
    def fullyContains(other: Range) = other.low >= low && other.high <= high
    def overlaps(other: Range) =
      other.low >= low && other.low <= high || other.high >= low && other.high <= high
  }
  object Range {
    lazy val rangeParser = (bigInt ~ (pchar('-') *> bigInt))
    lazy val commaSeperateRange = rangeParser ~ (pchar(',') *> rangeParser)
    def parseCommaSeperatedLine(line: String) =
      commaSeperateRange.parse(line).map {
        case (_, ((low1, high1), (low2, high2))) =>
          (Range(low1, high1), Range(low2, high2))
      }
  }

  val one = BigInt("1")
  val zero = BigInt("0")
  val partOne = FileIO
    .getInput[IO]("four.txt")
    .map(Range.parseCommaSeperatedLine(_))
    .collect { case Right((range1, range2)) =>
      if (range1.fullyContains(range2) || range2.fullyContains(range1)) {
        one
      } else {
        zero
      }
    }
    .compile
    .foldSemigroup

  val partTwo = FileIO
    .getInput[IO]("four.txt")
    .map(Range.parseCommaSeperatedLine(_))
    .collect { case Right((range1, range2)) =>
      if (range1.overlaps(range2) || range2.overlaps(range1)) {
        one
      } else {
        zero
      }
    }
    .compile
    .foldSemigroup

  val run = (for {
    outputOne <- partOne
    outputTwo <- partTwo
    _ <- IO.println(s"Part1: $outputOne")
    _ <- IO.println(s"Part2: $outputTwo")
  } yield ())
}
