import kotlinx.cli.ArgParser
import kotlinx.cli.ArgType

fun main(args: Array<String>) {
    val parser = ArgParser("bingo")
    val input by parser.argument(ArgType.String, description = "Input file")
    parser.parse(args)

    val scenario = Scenario.read(input)

    var winning: SimulationResult? = null
    var loosing: SimulationResult? = null
    scenario.boards.forEach {
        val result = it.simulate(scenario.numbers)
        if (winning == null || winning!!.iteration > result.iteration) winning = result
        if (loosing == null || loosing!!.iteration < result.iteration) loosing = result
    }

    println("Winning score is ${winning?.score}")
    println("Loosing score is ${loosing?.score}")
}