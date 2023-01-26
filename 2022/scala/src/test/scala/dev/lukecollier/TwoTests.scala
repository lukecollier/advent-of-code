package dev.lukecollier

import cats.syntax.all._
import weaver.SimpleMutableIOSuite
import cats.effect.IO

object TwoTests extends SimpleMutableIOSuite {
  import Two._
  test("can find the score from first play and results") {
    val input: fs2.Stream[IO, String] =
      fs2.Stream("A Y", "B X", "C Z").covary[IO]
    input.through(Two.partTwo).compile.foldMonoid.map { case (_, result) =>
      expect(result == BigInt("12"))
    }
  }
}
