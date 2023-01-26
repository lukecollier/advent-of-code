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

/** @link
  *   https://adventofcode.com/2022/day/2
  */
object Two extends IOApp.Simple {
  sealed trait Result {
    def score: BigInt
  }
  case object Win extends Result {
    def score = BigInt(6)
  }
  case object Lose extends Result {
    def score = BigInt(0)
  }
  case object Draw extends Result {
    def score = BigInt(3)
  }
  object Result {
    def fromString(str: String): Option[Result] = {
      str match {
        case "X" => Some(Lose)
        case "Y" => Some(Draw)
        case "Z" => Some(Win)
        case _   => None
      }
    }
  }

  sealed trait Play {
    def score: BigInt
    def vs(play: Play): Result
    def inverse(result: Result): Play
  }
  case object Rock extends Play {
    def score: BigInt = BigInt("1")
    def vs(play: Play): Result = play match {
      case Rock     => Draw
      case Paper    => Lose
      case Scissors => Win
    }
    def inverse(result: Result): Play = result match {
      case Draw => Rock
      case Lose => Scissors
      case Win  => Paper
    }
  }
  case object Paper extends Play {
    def score: BigInt = BigInt("2")
    def vs(play: Play): Result = play match {
      case Rock     => Win
      case Paper    => Draw
      case Scissors => Lose
    }
    def inverse(result: Result): Play = result match {
      case Draw => Paper
      case Lose => Rock
      case Win  => Scissors
    }
  }
  case object Scissors extends Play {
    def score: BigInt = BigInt("3")
    def vs(play: Play): Result = play match {
      case Rock     => Lose
      case Paper    => Win
      case Scissors => Draw
    }
    def inverse(result: Result): Play = result match {
      case Draw => Scissors
      case Lose => Paper
      case Win  => Rock
    }
  }
  object Play {
    def fromString(str: String): Option[Play] = str match {
      case "A" | "X" => Some(Rock)
      case "B" | "Y" => Some(Paper)
      case "C" | "Z" => Some(Scissors)
      case _         => None
    }
  }

  case class Game(playerOne: Play, playerTwo: Play) {
    def points: (BigInt, BigInt) =
      (
        playerOne.vs(playerTwo).score + playerOne.score,
        playerTwo.vs(playerOne).score + playerTwo.score
      )
  }
  object Game {
    def parse2(str: String): Option[Game] = {
      val ls =
        str
          .split(" ")
          .filterNot(_.trim().isEmpty())
          .toList
      (
        ls.get(0).flatMap(Play.fromString(_)),
        ls.get(1).flatMap(Result.fromString(_))
      ) match {
        case (Some(firstPlay), Some(gameResult)) => {
          val secondPlay = firstPlay.inverse(gameResult)
          Some(Game(firstPlay, secondPlay))
        }
        case _ => None
      }
    }
    def parse(str: String): Option[Game] = {
      val ls =
        str
          .split(" ")
          .filterNot(_.trim().isEmpty())
          .map(Play.fromString(_))
          .toList
      (ls.get(0).flatten, ls.get(1).flatten) match {
        case (Some(firstPlay), Some(secondPlay)) => {
          Some(Game(firstPlay, secondPlay))
        }
        case _ => None
      }
    }
  }

  val partOne = FileIO
    .getInput[IO]("two.txt")
    .map(Game.parse(_))
    .collect { case Some(x) => x }
    .map(_.points)
    .compile
    .foldMonoid

  def partTwo: fs2.Stream[IO, String] => fs2.Stream[IO, (BigInt, BigInt)] =
    _.map(Game.parse2(_))
      .collect { case Some(x) => x }
      .map(_.points)

  val run = (for {
    output1 <- partOne
    output2 <- FileIO
      .getInput[IO]("two.txt")
      .through(partTwo)
      .compile
      .foldMonoid
    _ <- IO.println(s"Part1: $output1")
    _ <- IO.println(s"Part2: $output2")
  } yield ())
}
