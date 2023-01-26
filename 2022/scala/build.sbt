import Dependencies._

ThisBuild / scalaVersion     := "2.13.8"
ThisBuild / version          := "0.1.0-SNAPSHOT"
ThisBuild / organization     := "dev.lukecollier"
ThisBuild / organizationName := "lukecollier"

lazy val root = (project in file("."))
  .settings(
    name := "aoc",
    libraryDependencies ++= Seq(weaverTest % Test, catsEffect, fs2, fs2IO, catsParse),
    testFrameworks += new TestFramework("weaver.framework.CatsEffect")
  )

// See https://www.scala-sbt.org/1.x/docs/Using-Sonatype.html for instructions on how to publish to Sonatype.
