package dev.lukecollier

import cats.syntax.all._
import weaver.SimpleMutableIOSuite
import cats.effect.IO
import cats.parse.Parser

object EightTests extends SimpleMutableIOSuite {
  import Eight._
  pureTest("tree in middle is visible") {
    val forest = Forest(
      // format: off
      List(
        1, 1, 1,
        2, 2, 2,
        3, 3, 3
      ),
    // format: on
      3
    )
    expect(forest.heightAt(3, 0) == None) and expect(
      forest.heightAt(0, 3) == None
    )
  }
  pureTest("tree in middle is visible") {
    val forest = Forest(
      // format: off
      List(
        1, 1, 1,
        2, 2, 2,
        3, 3, 3
      ),
    // format: on
      3
    )
    expect(forest.get(1, 1).map(_.visible) == Some(true))
  }

  pureTest("height is correct") {
    val forest = Forest(
      // format: off
      List(
        1, 2, 1,
        2, 1, 2,
        1, 2, 1
      ),
    // format: on
      3
    )
    expect(forest.height == 3)
  }

  pureTest("tree on edge is visible") {
    val forest = Forest(
      // format: off
      List(
        1, 2, 1,
        2, 1, 2,
        1, 2, 1
      ),
    // format: on
      3
    )
    expect(forest.get(1, 0).map(_.visible) == Some(true))
  }
  pureTest("tree in middle is not visible") {
    val forest = Forest(
      // format: off
      List(
        1, 2, 1,
        2, 1, 2,
        1, 2, 1
      ),
    // format: on
      3
    )
    expect(forest.get(1, 1).map(_.visible) == Some(false))
  }

  test("can parse example and find all visible trees") {
    Eight.puzzleOne(FileIO.getInput("eight.txt")).map { num =>
      expect(21 == num)
    }
  }

  test("can parse example finding tree with score of 8") {
    Eight.puzzleTwo(FileIO.getInput("eight.txt")).map { tree =>
      expect(tree == Tree(location = (3, 4), true, 8))
    }
  }

  pureTest("scenic score of tree on edge is 0") {
    val forest = Forest(
      // format: off
      List(
        1, 2, 1,
        2, 1, 2,
        1, 2, 1
      ),
    // format: on
      3
    )
    expect(forest.get(0, 1).map(_.scenicScore) == Some(0))
  }

  pureTest("scenic score of tree in middle is 1") {
    val forest = Forest(
      // format: off
      List(
        1, 2, 1,
        2, 1, 2,
        1, 2, 1
      ),
    // format: on
      3
    )
    expect(forest.get(1, 1).map(_.scenicScore) == Some(1))
  }

  pureTest("scenic score of tree in middle is 1") {
    val forest = Forest(
      // format: off
      List(
        1, 0, 1,
        2, 1, 0,
        1, 2, 1
      ),
      // format: on
      3
    )
    expect(forest.get(1, 1).map(_.scenicScore) == Some(1))
  }

  pureTest("scenic score of tree in middle is 16") {
    val forest = Forest(
      // format: off
      List(
        0, 0, 1, 0, 0,
        0, 0, 2, 0, 0,
        1, 2, 9, 2, 1,
        0, 0, 2, 0, 0,
        0, 0, 1, 0, 0,
      ),
      // format: on
      5
    )
    val forestGet = forest.get(2, 2)
    expect(forestGet.map(_.scenicScore) == Some(16))
  }

  pureTest("scenic score of tree in middle is 16") {
    val forest = Forest(
      // format: off
      List(
        3, 0, 3, 7, 3,
        2, 5, 5, 1, 2,
        6, 5, 3, 3, 2,
        3, 3, 5, 4, 9,
        3, 5, 3, 9, 0,
      ),
      // format: on
      5
    )
    val forestGet = forest.get(2, 3)
    expect(forestGet.map(_.scenicScore) == Some(8))
  }

}
