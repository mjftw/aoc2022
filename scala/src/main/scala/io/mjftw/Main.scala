package io.mjftw

import io.mjftw.Day4

import cats.implicits._
import cats.effect.IOApp
import cats.effect.IO
import cats.effect.ExitCode

object Main extends IOApp {

  def run(args: List[String]): IO[ExitCode] = {
    args match {
      case day4InputPath :: _ =>
        Day4.solve(day4InputPath).as(ExitCode.Success)
      case _ => IO.println("Must provide day 4 input path").as(ExitCode.Error)
    }
  }
}
