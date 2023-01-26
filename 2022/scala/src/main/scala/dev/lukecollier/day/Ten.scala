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
import scala.annotation.tailrec

/** @link
  *   https://adventofcode.com/2022/day/10
  */
object Ten extends LineStringDay[Int, Unit]() {
  override def resourceFileName = "ten.txt"

  sealed trait Instruction {
    def cycles: Int
  }
  case class AddX(value: BigInt) extends Instruction {
    val cycles = 2
  }
  case object NoOp extends Instruction {
    val cycles = 1
  }

  object Instruction {
    def addxP: Parser[AddX] =
      (Parser.string("addx ") *> cats.parse.Numbers.bigInt).map(AddX.apply(_))
    def noopP =
      Parser.string("noop").as(NoOp)
    def instructionP: Parser[Instruction] = addxP.orElse(noopP)
  }

  case class Cycle(cycle: Int, register: BigInt) {
    def signalStrength: Int = cycle * register.toInt
  }
  case class VirtualCPU(
      cycle: Int,
      register: BigInt,
      storedCycles: List[Cycle],
      instructions: List[Instruction]
  ) {
    val importantCycles = List(20, 60, 100, 140, 180, 220)

    def signalStrength: Int =
      storedCycles.map(_.signalStrength).sum

    @tailrec
    final def run: VirtualCPU = {
      def shouldSaveCycle(newCycle: Int) =
        (cycle to newCycle).exists(importantCycles.contains(_))
      def importantCycle(newCycle: Int) =
        (cycle to newCycle)
          .intersect(importantCycles)
          .headOption
          .orRaise(new RuntimeException("error: couldn't find correct cycle"))
          .get
      val currentInstruction = instructions.headOption
      val remaining = instructions.drop(1)
      currentInstruction match {
        case Some(addx @ AddX(value))
            if (shouldSaveCycle(cycle + addx.cycles)) =>
          this
            .copy(
              cycle = cycle + addx.cycles,
              register = register + value,
              storedCycles = this.storedCycles :+ Cycle(
                importantCycle(cycle + addx.cycles),
                register
              ),
              instructions = remaining
            )
            .run
        case Some(addx @ AddX(value)) =>
          this
            .copy(
              cycle = cycle + addx.cycles,
              register = register + value,
              instructions = remaining
            )
            .run
        case Some(noop @ NoOp) =>
          this.copy(cycle = cycle + noop.cycles, instructions = remaining).run
        case None => this
      }
    }

    def add(instruction: Instruction): VirtualCPU = {
      this.copy(instructions = this.instructions :+ instruction)
    }
  }
  object VirtualCPU {
    def empty =
      VirtualCPU(
        cycle = 0,
        register = 1,
        storedCycles = List.empty,
        instructions = List.empty
      )
  }

  override def puzzleOne =
    _.map(line => {
      val res = Instruction.instructionP.parseAll(line)
      if (res.isLeft) {
        println(line)
      }
      res
    })
      .evalTap {
        case Left(error) => IO.println(error)
        case _           => IO.unit
      }
      .fold(VirtualCPU.empty) {
        case (vcpu, Right(instruction)) => vcpu.add(instruction)
        case (vcpu, Left(_)) => {
          vcpu
        }
      }
      .map(_.run)
      .debug()
      .compile
      .lastOrError
      .map(_.signalStrength)

  override def puzzleTwo = _.compile.drain
}
