import java.io.File

class Scenario (
    val numbers: List<Int>,
    val boards: List<Board>
)  {
    companion object {
        private const val BINGO_SIZE = 5

        fun read(filePath: String): Scenario {
            val file = File(filePath)
            val lines = file.readLines()

            val numbers = lines[0].split(',').map {
                it.toInt()
            }

            val numberOfBoards = (lines.size - 1) / (BINGO_SIZE + 1)
            val boards = mutableListOf<Board>()
            for(boardNumber in 0 until numberOfBoards) {
                val boardFields = mutableListOf<Array<BoardField>>()
                for(rowIndex in 0 until BINGO_SIZE) {
                    val row = mutableListOf<BoardField>()
                    val line = lines[(boardNumber * (BINGO_SIZE+1))+rowIndex+2]
                    for(columnIndex in 0 until BINGO_SIZE) {
                        row.add(BoardField(
                            line.subSequence((3*columnIndex), (3*columnIndex)+2).toString().trim().toInt(),
                            false))
                    }
                    boardFields.add(row.toTypedArray())
                }
                boards.add(Board(boardFields.toTypedArray()))
            }

            return Scenario(numbers, boards)
        }
    }
}
