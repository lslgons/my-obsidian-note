# Scala 개요

> 출처: [Notion](https://app.notion.com/p/e8b32900f45a44f8adb6b3b190db345a)

# 스칼라

사용자의 요구에 따라 얼마든지 확장 가능한 언어 (Scalable)

객체지향과 함수형 프로그래밍 개념을 합쳐놓은 정적타입 언어

## 변수

var : 변수, 재할당 가능

val : (value), 재할당 불가, scala에서는 val사용 권장

```scala
val msg:String = "Hello world"
var msg2="Hello World"
```

## 함수

def로 함수정의

return 값이 없으면 Unit을 사용, 일종의 void

함수결과 타입이 Scala에서 타입추론이 가능하지만, 재귀함수의 경우 결과 타입을 반드시 명시

```scala
def max(a: Int, b: Int): Int = {
	if(a>b) a
	else b
}
```

## Array, List, Tuple

new 키워드를 사용하여 객체 인스턴스화, 이때 값을 파라미터화

Array는 인스턴스화 후 길이는 변경불가능하나 원소는 바꾸수 있음

scala.List의 경우 value 변경이 불가

Tuple은 리스트와 동일하게 변경이 불가능하나 각기 다른 타입의 원소를 넣을 수 있음

```scala
val greetingStrings = new Array[String](3)

greetingStrings(0)="Hello"
greetingStrings(1)=" "
greetingStrings(2)="World!\n"

val greetingStrings = Array("Hello", " ", "World!\n")
greetingStrings.update(2, "World!!!")

val numList=List(0,1,2)
val numList2=numList.updated(0,10)

val pair=(99,"Scala") //Tuple[Int,String]
println(pair._1)
println(pair._2)
```

## fold, map, filter

### fold

주어진 결합함수를 데이터 구조에 대해 재귀적으로 호출하여 결과값을 만들어 내는 고계함수(higher-order function)

```scala
def add(x:Int, y:Int):Int = x+y

val number_list=Array(1,2,3)

/**
리스트명.foldLeft, foldRight(최소값) {
	(최소값변수, 리스트_엘리먼트) => {
		//operations
	}
}
*/

number_list.foldLeft(0) {
	(sum,number) => add(sum,number)
}
// result: 6
```

### filter

리스트나 함수 내부에서 소괄호 내부의 조건을 만족하는 것들만 뽑아내는 것

리스트명.filter(x⇒필터링\_조건)

```scala
val number_list=List(1,2,3,4,5,6,7,8,9,0)
number_list.filter(x=>x>=3)
// val res0: List[Int] = List(3, 4, 5, 6, 7, 8, 9)
```

### map

리스트의 모든원소를 순회하는 매소드

```scala
val number_list=List(1,2,3,4,5,6,7,8,9,0)
number_list.map(x=>x*2)
val res1: List[Int] = List(2, 4, 6, 8, 10, 12, 14, 16, 18, 0)
```

## 클래스와 객체

### 클래스

멤버 : 필드+메서드

클래스 인스턴스화 시 Scala runtime은 해당 객체의 상태를 담아둘 메모리를 확보

클래스 내부 메서드 파라미터는 val ⇒ 변하지 않음

scala 접근자

- public (default) : Anywhere
- protected : 동일클래스, 상속 클래스에서만 접근 가능
- private : 동일 클래스에서만 접근가능

```scala
class CheckSumAccumulator {
	private var sum:Int = 0
	def add(x:Int):Unit ={
		sum=sum+x
	}
	def checkSum(): Int ={
		return ~(sum & 0xFF) +1
	}
}

val acc=new CheckSumAccumulator
```

### Singleton 객체

스칼라는 정적멤버(static)가 없음 → 싱글톤 객체 제공

object 예약어로 선언!!!

이미 선언된 동일명의 클래스가 있으며 이때 싱글톤을 동반객체(Companion Object), 클래스를 동반 클래스(Companion Class)로 불림

Companion Object-Class 간 private 멤버에 접근이 가능함

1급계층(Fist Class) : 제약없이 다룰 수 있는 대상, 인자로 넘길 수 있고 변수에 저장이 가능

독립개체 (Standalone object) : 동반 클래스가 없는 싱글톤 객체, 인스턴스 생성 불가하며 스칼라 앱 진입접이나 도구 메서드를 모아둘 때 사용

```scala
import scala.collection.mutable
// 싱글톤 객체 (object)
object ChecSumAccumulator {
	private val cache = mutable.Map.empty[String, Int]
    
  def calculate(s: String) :Int = {
  	if (cache.contains(s)) {
      	cache(s)
      }
      else {
      	val acc = new CheckSumAccumulator
          s.map(char => acc.add(char.toByte))
          
          val cs = acc.checkSum()
          cache += (s->cs)
          cs
      }
  ]
}

CheckSumAccumulator.calculate("Every value is an object")
```
