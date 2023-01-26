package dev.lukecollier.day

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
import dev.lukecollier.LineStringDay
import cats.Show

/** @link
  *   https://adventofcode.com/2022/day/9
  */
object Nine extends LineStringDay[Int, Int]() {
  case class Rope(rope: List[Knot]) {
    def emulate(move: Move) = {
      val moved = rope.headOption.map(_.move(move)).toList
      Rope(
        moved ++ rope.sliding2.map { case (hd, tl) =>
          tl.follow(hd.x, hd.y)
        }.toList
      )
    }
  }
  sealed trait RopeSegmant {
    val x: BigInt
    val y: BigInt
  }
  case class Knot(x: BigInt, y: BigInt) extends RopeSegmant {
    def move(move: Move) = move match {
      case Move.Up(amount)    => this.copy(y = y + amount)
      case Move.Down(amount)  => this.copy(y = y - amount)
      case Move.Right(amount) => this.copy(x = x + amount)
      case Move.Left(amount)  => this.copy(x = x - amount)
    }
    def angleTo(tx: Double, ty: Double): Double = {
      val angle = Math.toDegrees(Math.atan2(ty - y.toDouble, tx - x.toDouble))
      if (angle < 0) {
        angle + 360
      } else {
        angle
      }
    }
    def follow(tx: BigInt, ty: BigInt): Knot = {
      if (tx == x && ty == y) return this
      if (tx == x + 1 && ty == y) return this
      if (tx == x - 1 && ty == y) return this
      if (tx == x && ty == y + 1) return this
      if (tx == x && ty == y - 1) return this
      if (tx == x + 1 && ty == y + 1) return this
      if (tx == x - 1 && ty == y + 1) return this
      if (tx == x + 1 && ty == y - 1) return this
      if (tx == x - 1 && ty == y - 1) return this

      val angle = angleTo(tx.toDouble, ty.toDouble)
      def between(num: Double)(low: Double, high: Double) =
        num > low && num < high
      val isBetween = between(angle)(_, _)
      // todo: Angles don't quite align with the domain
      angle match {
        case 0.0 | 360.0 => this.copy(x = x + 1)
        // bottom left
        case _ if (isBetween(0.0, 90.0)) => this.copy(x = x + 1, y = y + 1)
        case 90.0                        => this.copy(y = y + 1)
        // bottom right
        case _ if (isBetween(90.0, 180.0)) => this.copy(x = x - 1, y = y + 1)
        case 180.0                         => this.copy(x = x - 1)
        // top right
        case _ if (isBetween(180.0, 270.0)) => this.copy(x = x - 1, y = y - 1)
        case 270.0                          => this.copy(y = y - 1)
        // top left
        case _ if (isBetween(270.0, 360.0)) => this.copy(x = x + 1, y = y - 1)
      }
    }
  }

  trait Move {
    def amount: BigInt
    def flatten: List[Move]
  }
  object Move {
    case class Up(amount: BigInt) extends Move {
      def flatten: List[Up] = (1 to amount.toInt).map(_ => Up(1)).toList
    }
    case class Down(amount: BigInt) extends Move {
      def flatten: List[Down] = (1 to amount.toInt).map(_ => Down(1)).toList
    }
    case class Left(amount: BigInt) extends Move {
      def flatten: List[Left] = (1 to amount.toInt).map(_ => Left(1)).toList
    }
    case class Right(amount: BigInt) extends Move {
      def flatten: List[Right] = (1 to amount.toInt).map(_ => Right(1)).toList
    }
    def upP = (pchar('U') ~ sp *> bigInt).map(Up(_))
    def downP = (pchar('D') ~ sp *> bigInt).map(Down(_))
    def leftP = (pchar('L') ~ sp *> bigInt).map(Left(_))
    def rightP = (pchar('R') ~ sp *> bigInt).map(Right(_))

    def moveP: Parser[Move] = upP | downP | leftP | rightP
  }

  case class History(move: Move, rope: Rope)
  case class RopeSimulation(
      rope: Rope,
      history: List[History]
  ) {

    def tailSize: Int = history.flatMap(_.rope.rope.tail).toSet.size

    def emulate(move: Move): RopeSimulation = {
      if (rope.rope.size > 0) {
        // println(rope)
      }
      val (newRope, h) = move.flatten.foldLeft(
        (rope, List.empty[History])
      ) {
        case ((ro, history), move) => {
          val res = ro.emulate(move)
          (res, history :+ History(move, res))
        }
      }
      RopeSimulation(newRope, history ++ h)
    }
  }
  object RopeSimulation {

    def from(x: BigInt, y: BigInt) = {
      val head = Knot(x, y)
      val tail = Knot(x, y)
      val rope = Rope(List(head, tail))
      RopeSimulation(rope, List.empty)
    }

    def ofLength(x: BigInt, y: BigInt)(amount: Int) = {
      val rope = Rope(List.fill(amount)(Knot(x, y)))
      RopeSimulation(rope, List.empty)
    }
  }
  override def resourceFileName = "nine.txt"
  override def puzzleOne = _.map(Move.moveP.parseAll(_))
    .fold(RopeSimulation.from(0, 0)) {
      case (acc, Right(move)) => acc.emulate(move)
      case (acc, Left(err)) => {
        System.err.println(s"WARNING: error while parsing $err")
        acc
      }
    }
    .map(_.tailSize)
    .compile
    .lastOrError

  override def puzzleTwo = _.map(Move.moveP.parseAll(_))
    .fold(RopeSimulation.ofLength(0, 0)(10)) {
      case (acc, Right(move)) => acc.emulate(move)
      case (acc, Left(err)) => {
        System.err.println(s"WARNING: error while parsing $err")
        acc
      }
    }
    .map(_.tailSize)
    .compile
    .lastOrError
}
