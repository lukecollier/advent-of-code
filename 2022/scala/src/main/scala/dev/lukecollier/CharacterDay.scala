package dev.lukecollier

import fs2.Stream
import cats.Show
import cats.effect.Sync
import cats.effect.{IOApp, IO}
import cats.implicits._
import cats.effect.std
import cats.syntax.show

abstract class CharacterDay[A: Show, B: Show] extends IOApp.Simple {
  def resourceFileName: String
  def puzzleOne: Stream[IO, Char] => IO[A]
  def puzzleTwo: Stream[IO, Char] => IO[B]

  override val run = (for {
    puzzleOutputOne <- puzzleOne(FileIO.getInputPerChar[IO](resourceFileName))
    puzzleOutputTwo <- puzzleTwo(FileIO.getInputPerChar[IO](resourceFileName))
    _ <- IO.println(show"Puzzle Output Two: $puzzleOutputOne")
    _ <- IO.println(show"Puzzle Output One: $puzzleOutputTwo")
  } yield ())
}
