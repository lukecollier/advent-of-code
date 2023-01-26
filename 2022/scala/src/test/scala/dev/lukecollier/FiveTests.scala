package dev.lukecollier

import cats.syntax.all._
import weaver.SimpleMutableIOSuite
import cats.effect.IO
import cats.parse.Parser

object FiveTests extends SimpleMutableIOSuite {
  import Five._
  pureTest("can get the heads of all stacks") {
    val crates =
      Crates(Map(1 -> List(Crate('A'), Crate('B')), 2 -> List(Crate('C'))))
    val expected = List(Some(Crate('B')), Some(Crate('C')))
    expect(crates.tops == expected)
  }
  pureTest("can parse a full line of crates") {
    val crateLine = "[A] [B] [C]"
    val parsed = Crates.cratesP.parse(crateLine)
    expect(parsed.isRight)
  }

  pureTest("can parse a partial line of crates") {
    val crateLine = "[A]     [C]"
    val parsed = Crates.cratesP.parse(crateLine)
    expect(parsed.isRight)
  }

  pureTest("can parse a partial line of control") {
    val line = "move 1 from 2 to 1"
    val parsed = Crates.moveP.parseAll(line)
    expect(parsed.isRight)
  }

  pureTest("can parse a partial line of control") {
    val line = "move 1 from 2 to 1"
    val parsed = Crates.moveP.parseAll(line)
    expect(parsed.isRight)
  }

  pureTest("can move a single crate") {
    val crates = Crates(Map(1 -> List(Crate('A')), 2 -> List(Crate('B'))))
    val exp = Crates(Map(1 -> List(), 2 -> List(Crate('B'), Crate('A'))))
    expect(crates.emulateCrateMover9000(Move(1, 1, 2)) == Some(exp))
  }

  pureTest("can move multiple crates") {
    val crates =
      Crates(Map(1 -> List(Crate('A'), Crate('C')), 2 -> List(Crate('B'))))
    val exp =
      Crates(Map(1 -> List(), 2 -> List(Crate('B'), Crate('C'), Crate('A'))))
    expect(crates.emulateCrateMover9000(Move(2, 1, 2)) == Some(exp))
  }

  pureTest("can move multiple crates") {
    val crates =
      Crates(Map(1 -> List(Crate('A'), Crate('C')), 2 -> List(Crate('B'))))
    val exp =
      Crates(Map(1 -> List(), 2 -> List(Crate('B'), Crate('C'), Crate('A'))))
    expect(crates.emulateCrateMover9000(Move(2, 1, 2)) == Some(exp))
  }

  pureTest("crates are moved one at a tie") {
    val crates =
      Crates(Map(1 -> List(Crate('A'), Crate('B'), Crate('C')), 2 -> List()))
    val exp =
      Crates(Map(1 -> List(), 2 -> List(Crate('C'), Crate('B'), Crate('A'))))
    expect(crates.emulateCrateMover9000(Move(3, 1, 2)) == Some(exp))
  }

  pureTest("crates are moved from the top of the list") {
    val crates =
      Crates(Map(1 -> List(Crate('A'), Crate('B'), Crate('C')), 2 -> List()))
    val exp =
      Crates(Map(1 -> List(Crate('A'), Crate('B')), 2 -> List(Crate('C'))))
    expect(crates.emulateCrateMover9000(Move(1, 1, 2)) == Some(exp))
  }

  test("moves apply to state correctly") {
    for {
      outputOne <- Five.partOne(
        FileIO
          .getInput[IO]("five.txt")
      )
    } yield expect(
      outputOne == Crates(
        Map(
          1 -> List(Crate('C')),
          2 -> List(Crate('M')),
          3 -> List(Crate('P'), Crate('D'), Crate('N'), Crate('Z'))
        )
      )
    )
  }

  test("can parse initial state") {
    for {
      outputOne <- Five.partOne(
        FileIO
          .getInput[IO]("five_state.txt")
      )
    } yield expect(
      outputOne.inner == Map(
        1 -> List(Crate('Z'), Crate('N')),
        2 -> List(Crate('M'), Crate('C'), Crate('D')),
        3 -> List(Crate('P'))
      )
    )
  }

}
