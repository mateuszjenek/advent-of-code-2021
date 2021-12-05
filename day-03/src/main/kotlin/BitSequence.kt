import kotlin.math.pow

class BitSequence(size: Int) {
    private val data: Array<Bit> = Array(size) { Bit(0) }

    fun at(index: Int): Bit {
        return data[index]
    }

    fun set(index: Int, value: Bit) {
        data[index] = value
    }

    fun toInt(): Int {
        var result = 0.0
        data.reversedArray().forEachIndexed { index, bit ->
            result += bit.value * 2.0.pow(index.toDouble())
        }
        return result.toInt()
    }
}
