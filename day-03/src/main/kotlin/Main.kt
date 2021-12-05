import kotlinx.cli.ArgParser
import kotlinx.cli.ArgType

fun main(args: Array<String>) {
    val parser = ArgParser("binary_diagnostic")
    val input by parser.argument(ArgType.String, description = "Input file")
    parser.parse(args)

    val report = DiagnosticReport.read(input)

    val gamma = report.getGammaRate()
    val epsilon = report.getEpsilonRate()
    val powerConsumption = PowerConsumption.calculate(gamma, epsilon)

    val oxygen = report.getOxygenGeneratorRate()
    val co2Scrubber = report.getCO2ScrubberRate()
    val lifeSupportRate = LifeSupportRate.calculate(oxygen, co2Scrubber)

    println("Power consumption: $powerConsumption Life support rating: $lifeSupportRate")
}