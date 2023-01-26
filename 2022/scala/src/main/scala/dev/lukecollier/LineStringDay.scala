package dev.lukecollier

import fs2.Stream
import cats.Show
import cats.effect.Sync
import cats.effect.{IOApp, IO}
import cats.implicits._
import cats.effect.std
import cats.syntax.show
import fs2.concurrent.Topic
import cats.effect.kernel.Resource

abstract class LineStringDay[A: Show, B: Show] extends IOApp.Simple {
  def resourceFileName: String
  def puzzleOne: Stream[IO, String] => IO[A]
  def puzzleTwo: Stream[IO, String] => IO[B]

  def puzzleResources = for {
    topic <- Resource.eval(Topic[IO, String])
    _ <- FileIO
      .getInput[IO](resourceFileName)
      .through(topic.publish)
      .compile
      .drain
      .background
    streamOne <- topic.subscribeAwaitUnbounded
    streamTwo <- topic.subscribeAwaitUnbounded
    puzzleOneConcurrent <- puzzleOne(streamOne).background
    puzzleTwoConcurrent <- puzzleTwo(streamTwo).background
  } yield (puzzleOneConcurrent, puzzleTwoConcurrent)

  override val run = puzzleResources.use {
    case (one, two) => (
      for {
        puzzleOutputOne <- one
        puzzleOutputTwo <- two
        outputOne <- puzzleOutputOne.embedError
        outputTwo <- puzzleOutputTwo.embedError
        _ <- IO.println(show"Puzzle Output One: $outputOne")
        _ <- IO.println(show"Puzzle Output Two: $outputTwo")
      } yield ()
    )
  }
}
