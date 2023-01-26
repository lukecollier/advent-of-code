package dev.lukecollier

import cats.syntax.all._
import weaver.SimpleMutableIOSuite
import cats.effect.IO
import cats.parse.Parser

object SevenTests extends SimpleMutableIOSuite {
  import Seven._
  pureTest("can parse a ls command") {
    val line = "$ cd twnj"
    val expected = ChangeDirectories("twnj")
    expect(Command.commandP.parseAll(line) == Right(expected))
  }

}
