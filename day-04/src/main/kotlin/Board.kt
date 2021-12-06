class Board (
    private val board: Array<Array<BoardField>>
    ) {
    fun simulate(numbers: List<Int>): SimulationResult {
        var iteration = 0
        for(number in numbers) {
            iteration++
            val coordinates = find(number) ?: continue
            board[coordinates.first][coordinates.second].active = true
            if(isBingo()) break
        }
        return SimulationResult(
            iteration = iteration,
            score = getScore(numbers[iteration-1]),
            bingo = iteration != numbers.size
        )
    }

    fun calculate(numbers: List<Int>): Int {
        var iteration = -1
        for(number in numbers) {
            iteration++
            val coordinates = find(number) ?: continue
            board[coordinates.first][coordinates.second].active = true
            if(isBingo()) break
        }
        return iteration
    }

    fun getScore(winningNumber: Int): Int {
        var inactiveSum = 0
        board.forEach {
            it.forEach {
                if (!it.active) inactiveSum += it.value
            }
        }
        return inactiveSum * winningNumber
    }

    private fun find(number: Int): Pair<Int, Int>? {
        board.forEachIndexed { row, line ->
            line.forEachIndexed { column, boardField ->
                if (boardField.value == number)
                    return Pair(row, column)
            }
        }
        return null
    }

    private fun isBingo(): Boolean {
        for (firstIndex in board.indices) {
            var isBingoRow = true
            var isBingoColumn = true
            for (secondIndex in board.indices) {
                if (!board[firstIndex][secondIndex].active) isBingoRow = false
                if (!board[secondIndex][firstIndex].active) isBingoColumn = false
            }
            if (isBingoRow || isBingoColumn) return true
        }
        return false
    }
}