
void FUN_14541d7d0(longlong param_1)

{
  longlong *plVar1;
  undefined4 *puVar2;
  undefined8 *puVar3;
  longlong lVar4;
  undefined4 *puVar5;
  undefined8 *puVar6;
  longlong *plVar7;
  code *pcVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  longlong *plVar11;
  longlong *plVar12;
  longlong lVar13;
  undefined8 uVar14;
  longlong lVar15;
  undefined4 *puVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  longlong *plVar20;
  bool bVar21;
  
  plVar7 = *(longlong **)(param_1 + 0x1a8);
  if ((longlong *)*plVar7 != plVar7) {
    plVar1 = (longlong *)(param_1 + 0x230);
    plVar20 = (longlong *)*plVar7;
    do {
      FUN_145676fe0(plVar20 + 0xb);
      lVar13 = FUN_140ab6b70(plVar1,plVar20 + 4);
      if (lVar13 != *plVar1) {
        uVar14 = FUN_140132910(plVar1,lVar13);
        FUN_1411670c0(plVar1,uVar14);
      }
      plVar11 = (longlong *)plVar20[2];
      if (*(char *)(plVar20[2] + 0x19) == '\0') {
        do {
          plVar12 = plVar11;
          plVar11 = (longlong *)*plVar12;
        } while (*(char *)(*plVar12 + 0x19) == '\0');
      }
      else {
        do {
          plVar12 = (longlong *)plVar20[1];
          if (*(char *)((longlong)plVar12 + 0x19) != '\0') break;
          bVar21 = plVar20 == (longlong *)plVar12[2];
          plVar20 = plVar12;
        } while (bVar21);
      }
      plVar20 = plVar12;
    } while (plVar12 != plVar7);
  }
  lVar13 = *(longlong *)(param_1 + 0x240);
  if (lVar13 == 0) {
    return;
  }
  *(undefined8 *)(lVar13 + 8) = 0;
  *(undefined4 *)(lVar13 + 0x10) = 0;
  puVar3 = *(undefined8 **)(lVar13 + 0x18);
  *(undefined8 *)puVar3[1] = 0;
  puVar3 = (undefined8 *)*puVar3;
  while (puVar3 != (undefined8 *)0x0) {
    puVar6 = (undefined8 *)*puVar3;
    uVar17 = puVar3[6];
    if (0xf < uVar17) {
      lVar4 = puVar3[3];
      uVar18 = uVar17 + 1;
      lVar15 = lVar4;
      if (0xfff < uVar18) {
        lVar15 = *(longlong *)(lVar4 + -8);
        if (0x1f < (ulonglong)((lVar4 + -8) - lVar15)) {
          _invoke_watson(0,0,0,0,0);
          pcVar8 = (code *)swi(3);
          (*pcVar8)();
          return;
        }
        uVar18 = uVar17 + 0x28;
      }
      FUN_14e934b00(lVar15,uVar18);
    }
    puVar3[5] = 0;
    puVar3[6] = 0xf;
    *(undefined1 *)(puVar3 + 3) = 0;
    FUN_14e934b00(puVar3,0x58);
    puVar3 = puVar6;
  }
  lVar4 = *(longlong *)(lVar13 + 0x18);
  *(longlong *)lVar4 = lVar4;
  *(longlong *)(lVar4 + 8) = lVar4;
  *(undefined8 *)(lVar13 + 0x20) = 0;
  if (*(longlong *)(lVar13 + 0x38) != 0) {
    puVar3 = *(undefined8 **)(lVar13 + 0x30);
    *(undefined8 *)puVar3[1] = 0;
    puVar3 = (undefined8 *)*puVar3;
    while (puVar3 != (undefined8 *)0x0) {
      puVar6 = (undefined8 *)*puVar3;
      uVar17 = puVar3[6];
      if (0xf < uVar17) {
        lVar4 = puVar3[3];
        uVar18 = uVar17 + 1;
        lVar15 = lVar4;
        if (0xfff < uVar18) {
          lVar15 = *(longlong *)(lVar4 + -8);
          if (0x1f < (ulonglong)((lVar4 + -8) - lVar15)) {
            _invoke_watson(0,0,0,0,0);
            pcVar8 = (code *)swi(3);
            (*pcVar8)();
            return;
          }
          uVar18 = uVar17 + 0x28;
        }
        FUN_14e934b00(lVar15,uVar18);
      }
      puVar3[5] = 0;
      puVar3[6] = 0xf;
      *(undefined1 *)(puVar3 + 3) = 0;
      FUN_14e934b00(puVar3,0x50);
      puVar3 = puVar6;
    }
    lVar4 = *(longlong *)(lVar13 + 0x30);
    *(longlong *)lVar4 = lVar4;
    *(longlong *)(lVar4 + 8) = lVar4;
    *(undefined8 *)(lVar13 + 0x38) = 0;
    puVar16 = *(undefined4 **)(lVar13 + 0x40);
    puVar5 = *(undefined4 **)(lVar13 + 0x48);
    if (puVar16 != puVar5) {
      uVar9 = *(undefined4 *)(lVar13 + 0x30);
      uVar10 = *(undefined4 *)(lVar13 + 0x34);
      uVar17 = (longlong)puVar5 + (-8 - (longlong)puVar16);
      if (0x17 < uVar17) {
        uVar17 = (uVar17 >> 3) + 1;
        uVar18 = uVar17 & 0xfffffffffffffffc;
        uVar19 = 0;
        do {
          puVar2 = puVar16 + uVar19 * 2;
          *puVar2 = uVar9;
          puVar2[1] = uVar10;
          puVar2[2] = uVar9;
          puVar2[3] = uVar10;
          puVar2 = puVar16 + uVar19 * 2 + 4;
          *puVar2 = uVar9;
          puVar2[1] = uVar10;
          puVar2[2] = uVar9;
          puVar2[3] = uVar10;
          uVar19 = uVar19 + 4;
        } while (uVar18 != uVar19);
        puVar16 = puVar16 + uVar18 * 2;
        if (uVar17 == uVar18) {
          return;
        }
      }
      do {
        *puVar16 = uVar9;
        puVar16[1] = uVar10;
        puVar16 = puVar16 + 2;
      } while (puVar16 != puVar5);
    }
  }
  return;
}

