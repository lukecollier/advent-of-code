package dev.lukecollier

import cats.syntax.all._
import weaver.SimpleMutableIOSuite

object OneTests extends SimpleMutableIOSuite {
  import One._
  pureTest("can fill a top three container") {
    val oneTwoAndThree = new TopThree(Array(1, 2, 3))
    val twoThreeAndFour = oneTwoAndThree.max(BigInt(4))
    expect(twoThreeAndFour.toList == List(4, 3, 2))
  }
}
