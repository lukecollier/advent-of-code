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
import cats.kernel.Semigroup
import scala.concurrent.duration._
import cats.parse.Parser
import cats.parse.Numbers.{digits, bigInt}
import cats.parse.Parser.{char => pchar, anyChar, charIn}
import cats.parse.Rfc5234.{sp, cr, crlf}
import scala.collection.mutable.Stack
import cats.instances.char
import cats.data.NonEmptyList
import cats.parse.Rfc5234
import fs2.Chunk
import cats.Foldable
import cats.effect.std

/** @link
  *   https://adventofcode.com/2022/day/7
  * @note
  *   fun hard one, bit finnicky and performance could be improved drastically
  *   1. Currently the col and row search accumulates all possibilities then
  *      finds the max. We can improve this by streaming outwards from the
  *      search coord and finding the
  */
case class Tree(location: (Int, Int), visible: Boolean, scenicScore: Int)
object Tree {
  implicit val showTree = cats.Show.fromToString[Tree]
}
object Eight extends LineStringDay[Int, Tree]() {
  override def resourceFileName = "eight.txt"
  case class Forest(heights: List[Int], width: Int) {
    def heightAt(x: Int, y: Int): Option[Int] = if (
      x >= 0 && x < width && y >= 0 && y < height
    ) {
      heights.get(x + (y * width))
    } else {
      None
    }
    def unsafeHeightAt(x: Int, y: Int): Int = heightAt(x, y).get
    def get(x: Int, y: Int): Option[Tree] = heightAt(x, y).map { treeHeight =>
      val up = LazyList
        .range(0, y)
        .reverse
        .lazyZip(LazyList.from(1))
        .collectFirst {
          case (dy, idx) if (unsafeHeightAt(x, dy) >= treeHeight) =>
            idx
        }
      val down = LazyList
        .range(y + 1, height)
        .lazyZip(LazyList.from(1))
        .collectFirst {
          case (dy, idx) if (unsafeHeightAt(x, dy) >= treeHeight) =>
            idx
        }
      val left = LazyList
        .range(0, x)
        .reverse
        .lazyZip(LazyList.from(1))
        .collectFirst {
          case (dx, idx) if (unsafeHeightAt(dx, y) >= treeHeight) =>
            idx
        }
      val right = LazyList
        .range(x + 1, width)
        .lazyZip(LazyList.from(1))
        .collectFirst {
          case (dx, idx) if (unsafeHeightAt(dx, y) >= treeHeight) =>
            idx
        }

      val all = List(left, up, right, down)
      val counts = List(
        left.getOrElse(x),
        up.getOrElse(y),
        right.getOrElse((width - 1) - x),
        down.getOrElse((height - 1) - y)
      )

      val isVisible = all.exists(_.isEmpty)
      val score = if (x == 0 || x == width - 1 || y == 0 || y == height - 1) {
        0
      } else {
        counts
          .reduceLeftOption(_ * _)
          .getOrElse(0)
      }
      Tree((x + 1, y + 1), isVisible, score)
    }

    def reduceLeft[B >: Tree](f: (B, Tree) => B) = (0 until width)
      .flatMap { x =>
        (0 until height)
          .map { y =>
            get(x, y)
          }
          .collect {
            case Some(v) => v
            case None => throw new RuntimeException("todo// this cant happen")
          }
      }
      .reduceLeft[B](f)

    def fold[A, S](s: S)(f: (S, Tree) => S) = (0 until width)
      .flatMap { x =>
        (0 until height)
          .map { y =>
            get(x, y)
          }
          .collect {
            case Some(v) => v
            case None => throw new RuntimeException("todo// this cant happen")
          }
      }
      .foldLeft[S](s)(f)

    def merge(other: Forest) =
      this.copy(heights = this.heights ++ other.heights)

    def height(): Int =
      heights.length / width
  }
  object Forest {
    def forestP =
      (Rfc5234.digit
        .map(_.asDigit))
        .rep
        .map(heights => Forest(heights.toList, heights.length))
  }
  override def puzzleOne =
    _.evalMapFilter(line =>
      Forest.forestP.parseAll(line) match {
        case Left(error) =>
          std
            .Console[IO]
            .errorln(s"problem while parsing '$line' expected '$error' occured")
            .as(None)
        case result @ Right(_) => IO.pure(result.toOption)
      }
    ).reduce((forest: Forest, other: Forest) => forest.merge(other))
      .map(
        _.fold(0)((acc, tree) =>
          if (tree.visible) {
            acc + 1
          } else {
            acc
          }
        )
      )
      .compile
      .lastOrError

  override def puzzleTwo =
    _.evalMapFilter(line =>
      Forest.forestP.parseAll(line) match {
        case Left(error) =>
          std
            .Console[IO]
            .errorln(s"problem while parsing '$line' expected '$error' occured")
            .as(None)
        case result @ Right(_) => IO.pure(result.toOption)
      }
    ).reduce(_ merge _)
      .map(
        _.reduceLeft[Tree] {
          case (acc, tree) if (tree.scenicScore > acc.scenicScore) =>
            tree
          case (acc, _) => acc
        }
      )
      .compile
      .lastOrError
}
