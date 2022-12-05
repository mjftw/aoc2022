package io.mjftw

import cats.implicits._
import cats.MonadError
import cats.effect.IO
import fs2.{Stream, text}
import fs2.io.file.{Files, Path}
import scala.util.matching.Regex

class DecodeError(msg: String) extends Throwable(msg)

case class Bounds(lower: Int, upper: Int) {
  def contains(other: Bounds): Boolean =
    lower <= other.lower && upper >= other.upper
}

object Day4 {

  def solve(inputPath: String): IO[Unit] = {

    val answerPart1 = Files[IO]
      .readUtf8Lines(Path(inputPath))
      .map(lineToBounds)
      .rethrow
      .collect {
        case ((bounds1, bounds2))
            if bounds1.contains(bounds2) || bounds2.contains(bounds1) =>
          1

      }
      .fold(0)(_ + _)
      .compile
      .toList
      .map(_.head)

    answerPart1.flatMap(answer => IO.println(s"Day 4 part 1 answer: $answer"))
  }

  private def lineToBounds(
      line: String
  ): Either[DecodeError, (Bounds, Bounds)] = {

    val pattern: Regex = "([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)".r

    line match {
      case pattern(lower1, upper1, lower2, upper2) =>
        (
          Bounds(lower1.toInt, upper1.toInt),
          Bounds(lower2.toInt, upper2.toInt)
        ).asRight
      case _ => Left(new DecodeError(s"$line does not match regex: $pattern"))
    }
  }
}
