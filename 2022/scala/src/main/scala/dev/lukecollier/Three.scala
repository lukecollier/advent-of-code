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

/** @link
  *   https://adventofcode.com/2022/day/3
  */
object Three extends IOApp.Simple {
  case class Item(priority: Int)
  object Item {
    implicit val itemSemigroup = new Semigroup[Item] {
      override def combine(x: Item, y: Item): Item = Item(
        x.priority + y.priority
      )
    }
    val lowercaseLowerBound = ('a' - 1).toInt
    val lowercaseUpperBound = ('z' - lowercaseLowerBound).toInt
    def fromUtf8Char(char: Char): Option[Item] = if (char.isUpper) {
      Some(
        Item(char.toInt - ('A' - 1).toInt + lowercaseUpperBound)
      )
    } else if (char.isLower) {
      Some(Item(char.toInt - lowercaseLowerBound))
    } else {
      None
    }
  }
  val partOne = FileIO
    .getInput[IO]("three.txt")
    .map(line => {
      val rucksack = line.toList
      val (compartmentOne, compartmentTwo) =
        rucksack.splitAt(rucksack.length / 2)
      compartmentOne
        .intersect(compartmentTwo)
        .map(Item.fromUtf8Char(_))
        .headOption
        .flattenOption
    })
    .compile
    .foldSemigroup
    .map(_.flattenOption)

  val partTwo = FileIO
    .getInput[IO]("three.txt")
    .map(_.toList)
    .groupWithin(3, 1.minute)
    .map(_.reduceLeftOption { case (a, b) =>
      a.intersect(b).distinct
    })
    .map(_.map(_.headOption.map(Item.fromUtf8Char(_))))
    .compile
    .foldSemigroup
    .map(_.flattenOption.flattenOption.flattenOption)

  val run = (for {
    outputOne <- partOne
    outputTwo <- partTwo
    _ <- IO.println(s"Part1: ${outputOne.get.priority}")
    _ <- IO.println(s"Part2: ${outputTwo.get.priority}")
  } yield ())
}
