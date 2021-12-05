import java.io.File

class DiagnosticReport(private val data: Array<BitSequence>) {
    companion object {
        private const val BIT_SEQUENCE_SIZE = 12

        fun read(filePath: String): DiagnosticReport {
            val file = File(filePath)
            val data = mutableListOf<BitSequence>()
            file.forEachLine {
                val bitSequence = BitSequence(BIT_SEQUENCE_SIZE)
                it.forEachIndexed { index, value ->
                    bitSequence.set(index, Bit(value.toString().toInt()))
                }
                data.add(bitSequence)
            }
            return DiagnosticReport(data.toTypedArray())
        }

        private fun getTheMostCommonBitInColumn(data: Array<BitSequence>, column: Int): Bit {
            var zeroCounts = 0
            var oneCounts = 0
            data.forEach {
                if (it.at(column) == Bit(1)) oneCounts++
                else zeroCounts++
            }
            if (zeroCounts <= oneCounts)
                return Bit(1)
            return Bit(0)
        }

        private fun filterBitSequencesByColumn(data: Array<BitSequence>, filter: Bit, column: Int): Array<BitSequence> {
            val list = mutableListOf<BitSequence>()
            data.forEach {
                if (it.at(column) == filter) list.add(it)
            }
            return list.toTypedArray()
        }
    }

    fun getGammaRate(): Int {
        val sequence = BitSequence(BIT_SEQUENCE_SIZE)
        for (index in (0 until BIT_SEQUENCE_SIZE)) {
            val bit = getTheMostCommonBitInColumn(data, index)
            sequence.set(index, bit)
        }
        return sequence.toInt()
    }

    fun getEpsilonRate(): Int {
        val sequence = BitSequence(BIT_SEQUENCE_SIZE)
        for (index in (0 until BIT_SEQUENCE_SIZE)) {
            val bit = getTheMostCommonBitInColumn(data, index)
            sequence.set(index, bit.reversed())
        }
        return sequence.toInt()
    }

    fun getOxygenGeneratorRate(): Int {
        var filteredData = data.clone()
        for (index in (0 until BIT_SEQUENCE_SIZE)) {
            if (filteredData.size == 1) break
            val bit = getTheMostCommonBitInColumn(filteredData, index)
            filteredData = filterBitSequencesByColumn(filteredData, bit, index)
        }
        return filteredData[0].toInt()
    }

    fun getCO2ScrubberRate(): Int {
        var filteredData = data.clone()
        for (index in (0 until BIT_SEQUENCE_SIZE)) {
            if (filteredData.size == 1) break
            val bit = getTheMostCommonBitInColumn(filteredData, index)
            filteredData = filterBitSequencesByColumn(filteredData, bit.reversed(), index)
        }
        return filteredData[0].toInt()
    }
}
