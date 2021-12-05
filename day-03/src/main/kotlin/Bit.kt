data class Bit(val value: Int) {
    init {
        require(value == 0 || value == 1)
    }

    fun reversed(): Bit {
        return Bit((value + 1) % 2)
    }
}
