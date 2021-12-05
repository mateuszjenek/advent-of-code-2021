class LifeSupportRate {
    companion object {
        fun calculate(oxygenGeneratorRate: Int, co2ScrubberRate: Int): Int {
            return oxygenGeneratorRate * co2ScrubberRate
        }
    }
}