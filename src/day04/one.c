#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

int* read_int_array(const char* separator) {
    char readBuffer[1024];
    fgets(readBuffer, 1023, stdin);
    char* cursor = readBuffer - 1;
    int resultCount = 0;
    int* result = malloc(sizeof(int));
    do {
        cursor++;
        resultCount++;
        result = realloc(result, resultCount * sizeof(int));
        result[resultCount - 1] = atoi(cursor);
        cursor = strstr(cursor + 1, separator);
    } while (cursor);
    if (separator == ",") {
        result = realloc(result, (resultCount + 1) * sizeof(int));
        result[resultCount] = -1;
    }
    return result;
}

int* read_board(int* into) {
    memset(into, 0, 10 * 5 * sizeof(int));
    for (int row=0; row<5; row++) {
        int* line = read_int_array(" ");
        for(int col=0; col < 5; col++) into[row * 5 + col] = line[col];
        free(line);
    }
    
    return into;
}
int* read_boards() {
    int resultCount = 0;
    int* result = NULL;
    char readBuffer[1024];
    while(fgets(readBuffer, 1023, stdin)) {
        resultCount++;
        result = realloc(result, resultCount * 10 * 5 * sizeof(int));
        assert(result != NULL);
        read_board(&result[ (resultCount - 1) * 10 * 5 ]);
    }
    result = realloc(result, (1 + resultCount * 10 * 5) * sizeof(int));
    assert(result != NULL);
    result[resultCount * 10 * 5] = -1;
    return result;
}
int board_get(int* board, int boardIndex, int row, int col) { return board[ boardIndex * 10 * 5 + row * 5 + col]; }
void board_set(int* board, int boardIndex, int row, int col, int val) { board[ boardIndex * 10 * 5 + row * 5 + col] = val; }
int board_marked(int* board, int boardIndex, int row, int col) { return board_get(board, boardIndex, row + 5, col); }
void board_mark(int* board, int boardIndex, int row, int col) { board_set(board, boardIndex, row + 5, col, 1); }
void board_draw(int* board, int boardIndex, int val) { for (int row = 0; row < 5; row++) for (int col = 0; col < 5; col++) if (val == board_get(board, boardIndex, row, col)) board_mark(board, boardIndex, row, col); }
void boards_draw(int* boards, int boardCount, int val) { for (int board = 0; board < boardCount; board++) board_draw(boards, board, val); }
void print_board(int* boards, int board) {
    for (int row = 0; row < 5; row++) {
        for (int col=0; col < 5; col++) printf("%2d ", board_get(boards, board, row, col));
        printf("\n");
        for (int col=0; col < 5; col++) if (board_marked(boards, board, row, col)) printf("-- "); else printf("   ");
        printf("\n");
    }
}
int board_is_winner(int* boards, int board) {
    for (int row = 0; row < 5; row++) {
        for (int col=0; col < 5; col++) if (!board_marked(boards, board, row, col)) goto next_row;
        return 1;
        next_row:
    }
    for (int col = 0; col < 5; col++) {
        for (int row=0; row < 5; row++) if (!board_marked(boards, board, row, col)) goto next_col;
        return 1;
        next_col:
    }

    return 0;
}
int board_score(int* boards, int board) {
    int result = 0;
    for (int row = 0; row < 5; row++) for (int col=0; col < 5; col++) if (!board_marked(boards, board, row, col)) result += board_get(boards, board, row, col);
    return result;
}

int main() {
    int* draw = read_int_array(",");
    int* boards = read_boards();
    int boardCount;
    for (boardCount=0; boards[boardCount] != -1; boardCount++);
    int boardWon[1024];
    memset(boardWon, 0, 1024 * sizeof(int));
    boardCount /= 10 * 5;
    int isFirst = 1;
    int lastBoardScore = 0;
    for (int i=0; draw[i] != -1; i++) {
        boards_draw(boards, boardCount, draw[i]);
        for (int board = 0; board < boardCount; board++) if (board_is_winner(boards, board) && !boardWon[board]) {
            boardWon[board] = 1;
            lastBoardScore = draw[i] * board_score(boards, board);
            if (isFirst) {
                printf("%d\n", draw[i] * board_score(boards, board));
                isFirst = 0;
            }
        }
    }
    printf("%d\n", lastBoardScore);
}