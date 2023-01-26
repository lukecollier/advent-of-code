package dev.lukecollier.day

import cats.syntax.all._
import weaver.SimpleMutableIOSuite
import cats.effect.IO
import cats.parse.Parser
import weaver.{Expectations, TestName}
import dev.lukecollier.FileIO

object NineTests extends SimpleMutableIOSuite {
  import Nine._
  pureTest("tail won't move when touching") {
    val (x, y) = (100, 100)

    val left = Knot(99, 100)
    val bottomLeft = Knot(99, 99)
    val topLeft = Knot(99, 101)
    val top = Knot(100, 101)
    val topRight = Knot(101, 101)
    val right = Knot(101, 100)
    val bottomRight = Knot(101, 99)
    val bottom = Knot(100, 99)
    val hidden = Knot(100, 100)

    expect(left.follow(x, y) == left) and
      expect(bottomLeft.follow(x, y) == bottomLeft) and
      expect(topLeft.follow(x, y) == topLeft) and
      expect(top.follow(x, y) == top) and
      expect(topRight.follow(x, y) == topRight) and
      expect(right.follow(x, y) == right) and
      expect(bottomRight.follow(x, y) == bottomRight) and
      expect(bottom.follow(x, y) == bottom) and
      expect(hidden.follow(x, y) == hidden)
  }

  pureTest("can parse a move order") {
    val parsed = Move.moveP.parseAll("L 1")
    val expected = Move.Left(BigInt("1"))
    expect(parsed == Right(expected))
  }

  pureTest("can move up 4") {
    val knot = Knot(0, 0).move(Move.Up(4))
    val expected = Knot(0, 4)
    expect(knot == expected)
  }

  pureTest("can move right 4") {
    val knot = Knot(0, 0).move(Move.Right(4))
    val expected = Knot(4, 0)
    expect(knot == expected)
  }

  pureTest("can move left 2") {
    val knot = Knot(0, 0).move(Move.Left(2))
    val expected = Knot(-2, 0)
    expect(knot == expected)
  }

  pureTest("can move down 2") {
    val knot = Knot(0, 0).move(Move.Down(2))
    val expected = Knot(0, -2)
    expect(knot == expected)
  }

  test("example can be ran") {
    puzzleOne(FileIO.getInput("nine.txt")).map { positions =>
      expect(positions == 13)
    }
  }

  pureTest("tail knot will move when not touching") {
    val tail = Knot(100, 100)
    val expected = Knot(101, 101)
    expect(tail.follow(102, 102) == expected)
  }

  pureTest("tail knot will move top right when not touching") {
    val tail = Knot(100, 100)
    val expected = Knot(100, 101)
    expect(tail.follow(100, 102) == expected)
  }

  pureTest("tail knot will move top right when not touching") {
    val tail = Knot(100, 100)
    val expected = Knot(100, 101)
    expect(tail.follow(100, 102) == expected)
  }

  pureTest("tail knot will move top right when not touching") {
    val tail = Knot(100, 100)
    val expected = Knot(101, 101)
    expect(tail.follow(101, 102) == expected)
  }

  pureTest("tail knot will move right when not touching") {
    val tail = Knot(100, 100)
    val expected = Knot(101, 100)
    expect(tail.follow(102, 100) == expected)
  }
}
