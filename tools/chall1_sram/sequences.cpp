#include "sram.cpp"

void hexprint(uint8 *w){
    for (int i=0; i<16; i++){
        printf("%.2x ", *(w+i));
    }
    putchar('\n');
}

int main(int argc, char **argv){
    fprintf(stderr, "Reading seeded SRAM data...\n");
    SRAM_Corruption corruption;
    corruption.loadInitialState("sram_start.dmp");
    fprintf(stderr, "Reading sprite behavior data...\n");
    fflush(stderr);
    corruption.loadSpriteBehavior("behavior_1f.txt");
    hexprint(corruption.sram + 0x7D0);

    for (size_t i = 0; i < 63; i++) {
        corruption.corrupt();
        printf("Iteration %zu:\n    ", i);
        hexprint(corruption.sram + 0x7D0);
    }

    FILE *fp = fopen("sram_noice.dmp", "wb");
    if (!fp) return 1;
    fwrite(corruption.sram, 1, SRAM_SIZE, fp);
    fclose(fp);
}
