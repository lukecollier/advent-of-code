package dev.lukecollier

import fs2.io.file.{Path, Files}
import cats.effect.{Concurrent, Sync}
import fs2.{Stream, text}
import scala.io.Source

object FileIO {
  def getInputPerChar[F[_]: Sync](
      path: String
  ): Stream[F, Char] = {
    fs2.io
      .readClassLoaderResource(path)
      .through(text.utf8.decode)
      .flatMap(str => Stream.fromIterator(str.toCharArray().toIterator, 1))
  }

  def getInput[F[_]: Sync](
      path: String
  ): Stream[F, String] = {
    fs2.io
      .readClassLoaderResource(path)
      .through(text.utf8.decode)
      .through(text.lines)
  }
}
