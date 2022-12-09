package io.mjftw

import cats.implicits._
import cats.MonadError
import cats.effect.IO
import fs2.{Stream, text}
import fs2.io.file.{Files, Path}
import scala.util.matching.Regex

object Day5 {
  case class Move(num: Int, from: Int, to: Int)

  object Move {
    def fromLine(line: String): Move = {
      val pattern = ".*([0-9]+).+([0-9]+).+([0-9]+).*".r

      line match {
        case pattern(num, from, to) =>
          Move(num.toInt, from.toInt - 1, to.toInt - 1)
      }
    }
  }

  type BoxStack = List[Char]
  case class BoxStacks(stacks: List[BoxStack]) {
    def applyMove(move: Move): IO[BoxStacks] = {
      IO.raiseWhen(stacks(move.from).length < move.num)(
        new RuntimeException(
          s"Cannot move ${move.num} from ${move.from} to ${move.to} as only ${stacks(move.from).length} crates available"
        )
      ).as(
        BoxStacks(
          stacks
            .updated(
              move.to,
              stacks(move.from).take(move.num).reverse ++ stacks(move.to)
            )
            .updated(move.from, stacks(move.from).drop(move.num))
        )
      )

    }
  }

  object BoxStacks {
    def fromLines(lines: List[String]): BoxStacks = {

      lines.reverse match {
        case numbersLine :: cratesLines =>
          val stackLineIndices =
            "[1-9]".r.findAllMatchIn(numbersLine).map(_.start).toList

          val numStacks = stackLineIndices.length

          BoxStacks(
            cratesLines
              .map(line => {
                line.padTo(numbersLine.length, "_").mkString
              })
              .map(getLineBoxChars(_, stackLineIndices))
              .transpose
              .map(_.takeWhile(_ != ' ').reverse)
          )
      }
    }

    private def getLineBoxChars(
        line: String,
        stackLineIndices: List[Int]
    ): List[Char] = {
      val lineVec = line.toVector
      stackLineIndices.map(index => lineVec(index))
    }
  }

  def solve(inputPath: String): IO[Unit] = {
    for {
      lines <- Files[IO]
        .readUtf8Lines(Path(inputPath))
        .compile
        .toList

      stacksLines = lines.takeWhile(line => line.trim.nonEmpty)

      boxStacks = BoxStacks.fromLines(stacksLines).pure[IO]

      moves = lines
        .drop(stacksLines.length + 1)
        .filter(line => line.trim.nonEmpty)
        .map(Move.fromLine)

      finalStacks <- moves.foldLeft(boxStacks) { (stacks, move) =>
        for {
          stacks <- stacks
          _ <- IO.println(
            s"${stacks.stacks.map(_.mkString).zipWithIndex}\n$move"
          )
          moved <- stacks.applyMove(move)
        } yield moved
      }

      topCrates = finalStacks.stacks
        .map(_.headOption.getOrElse(" "))
        .mkString

      _ <- IO.println(
        s"${finalStacks.stacks.map(_.mkString).zipWithIndex}\n"
      )

      _ <- IO.println(s"Day 5 part 1 answer: $topCrates")

    } yield ()

  }
}
