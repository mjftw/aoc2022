package io.mjftw

import io.mjftw.Day4

import cats.implicits._
import cats.effect.IOApp
import cats.effect.IO
import cats.effect.ExitCode

object Main extends IOApp {

  def run(args: List[String]): IO[ExitCode] = {
    args match {
      case "4" :: inputPath :: _ =>
        Day4.solve(inputPath).as(ExitCode.Success)
      case "5" :: inputPath :: _ =>
        Day5.solve(inputPath).as(ExitCode.Success)
      case _ =>
        IO.println("Must provide valid day and input path").as(ExitCode.Error)
    }
  }
}
