package dev.lukecollier

import cats.effect.{IOApp, IO}
import cats.syntax.all._

import cats.effect.Concurrent
import fs2.{hash, text}
import fs2.io.file.{Files, Path}
import cats.effect.ExitCode
import scala.util.Try
import cats.kernel.Eq

/** @link
  *   https://adventofcode.com/2022/day/1#part2
  */
object One extends IOApp.Simple {
  val partOne = FileIO
    .getInput[IO]("one.txt")
    .map(potentialNumber => Try(BigInt.apply(potentialNumber)).toOption)
    .split(_.isEmpty)
    .map(_.toList.foldA)
    .compile
    .fold(BigInt.apply("0")) {
      case (left, Some(right)) => left.max(right)
      case (left, None)        => left
    }

  class TopThree(numbers: Array[BigInt]) {
    def toList: List[BigInt] = numbers.toList
    def sum(): BigInt = numbers.sum
    def max(candidate: BigInt): TopThree =
      new TopThree(
        numbers.appended(candidate).sorted(Ordering[BigInt].reverse).take(3)
      )

    override def toString(): String =
      this.getClass.getSimpleName ++ "(" ++ numbers.mkString(",") ++ ")"
  }
  object TopThree {
    def empty() = new TopThree(Array.empty)
  }

  val partTwo = FileIO
    .getInput[IO]("one.txt")
    .map(potentialNumber => Try(BigInt(potentialNumber)).toOption)
    .split(_.isEmpty)
    .map(_.toList.foldA)
    .compile
    .fold(TopThree.empty) {
      case (left, Some(right)) => left.max(right)
      case (left, None)        => left
    }
    .debug()
    .map(_.sum)

  val run = (for {
    // Part One
    output1 <- partOne
    // Part Two
    output2 <- partTwo
    _ <- IO.println(s"Part1: $output1")
    _ <- IO.println(s"Part2: $output2")
  } yield ())
}
