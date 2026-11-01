
/* WARNING: Type propagation algorithm not settling */

bool FUN_146bdee80(longlong *param_1)

{
  longlong lVar1;
  code *pcVar2;
  longlong lVar3;
  undefined1 uVar4;
  char cVar5;
  int iVar6;
  undefined8 uVar7;
  longlong *plVar8;
  longlong lVar9;
  undefined8 *******pppppppuVar10;
  longlong *plVar11;
  ulonglong uVar12;
  undefined1 *puVar13;
  undefined1 *puVar14;
  ulonglong uVar15;
  undefined8 *puVar16;
  undefined8 *******pppppppuVar17;
  bool bVar18;
  undefined4 unaff_XMM6_Dc;
  undefined4 unaff_XMM6_Dd;
  undefined1 *puStack_108;
  longlong lStack_100;
  longlong lStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  ulonglong uStack_e0;
  undefined8 *******apppppppuStack_d0 [2];
  longlong lStack_c0;
  ulonglong uStack_b8;
  undefined8 *puStack_b0;
  undefined8 uStack_a8;
  undefined1 auStack_a0 [16];
  undefined1 auStack_90 [8];
  undefined8 uStack_88;
  undefined1 uStack_80;
  undefined7 uStack_7f;
  undefined8 *puStack_78;
  longlong lStack_70;
  ulonglong uStack_68;
  undefined8 *******apppppppuStack_60 [2];
  undefined4 uStack_50;
  undefined4 uStack_4c;
  ulonglong uStack_48;
  
  apppppppuStack_60[0] = (undefined8 *******)0xfffffffffffffffe;
  lVar1 = *param_1;
  if (*(longlong *)(lVar1 + 0x70) == 0) {
    return true;
  }
  plVar11 = (longlong *)(lVar1 + 0x60);
  uStack_50 = unaff_XMM6_Dc;
  uStack_4c = unaff_XMM6_Dd;
  cVar5 = FUN_14e931900(lVar1 + 0xf8);
  if (cVar5 == '\0') {
    uVar7 = FUN_14e9322b0(lVar1 + 0xf8,&UNK_14f516ebf);
    cVar5 = FUN_14e9319a0(uVar7);
    if (cVar5 != '\0') {
      func_0x00014e9329a0(uVar7,auStack_a0);
      func_0x00014e9329f0(uVar7,auStack_90);
      while( true ) {
        puVar14 = &uStack_80;
        cVar5 = func_0x00014e933b00(auStack_a0,auStack_90);
        if (cVar5 != '\0') break;
        uVar7 = func_0x00014e933ad0(auStack_a0);
        uVar7 = FUN_14e9322b0(uVar7,&UNK_14f5860fa);
        uVar7 = FUN_14e9322b0(uVar7,&UNK_14f56bd9d);
        lStack_f8 = 0;
        uStack_f0 = 0;
        uStack_e8 = 0;
        uStack_e0 = 0xf;
        FUN_14e931540(uVar7,puVar14,&lStack_f8);
        if (0xf < uStack_68) {
          puVar14 = (undefined1 *)CONCAT71(uStack_7f,uStack_80);
        }
        lStack_100 = lStack_70;
        puStack_108 = puVar14;
        FUN_1403e8120(apppppppuStack_d0,&puStack_108);
        uVar15 = uStack_b8;
        pppppppuVar17 = apppppppuStack_d0[0];
        plVar8 = plVar11;
        if (0xf < *(ulonglong *)(lVar1 + 0x78)) {
          plVar8 = *(longlong **)(lVar1 + 0x60);
        }
        pppppppuVar10 = apppppppuStack_d0;
        if (0xf < uStack_b8) {
          pppppppuVar10 = apppppppuStack_d0[0];
        }
        if (lStack_c0 == *(longlong *)(lVar1 + 0x70)) {
          if (*(longlong *)(lVar1 + 0x70) == 0) {
            bVar18 = true;
          }
          else {
            iVar6 = FUN_14eec62b0(pppppppuVar10,plVar8);
            bVar18 = iVar6 == 0;
          }
        }
        else {
          bVar18 = false;
        }
        if (0xf < uVar15) {
          uVar12 = uVar15 + 1;
          pppppppuVar10 = pppppppuVar17;
          if (0xfff < uVar12) {
            pppppppuVar10 = (undefined8 *******)pppppppuVar17[-1];
            if (0x1f < (ulonglong)((longlong)pppppppuVar17 + (-8 - (longlong)pppppppuVar10)))
            goto LAB_146bdf280;
            uVar12 = uVar15 + 0x28;
          }
          FUN_14e934b00(pppppppuVar10,uVar12);
        }
        if (0xf < uStack_68) {
          lVar3 = CONCAT71(uStack_7f,uStack_80);
          uVar15 = uStack_68 + 1;
          lVar9 = lVar3;
          if (0xfff < uVar15) {
            lVar9 = *(longlong *)(lVar3 + -8);
            if (0x1f < (ulonglong)((lVar3 + -8) - lVar9)) goto LAB_146bdf280;
            uVar15 = uStack_68 + 0x28;
          }
          FUN_14e934b00(lVar9,uVar15);
        }
        lStack_70 = 0;
        uStack_68 = 0xf;
        uStack_80 = 0;
        if (0xf < uStack_e0) {
          uVar15 = uStack_e0 + 1;
          lVar9 = lStack_f8;
          if (0xfff < uVar15) {
            lVar9 = *(longlong *)(lStack_f8 + -8);
            if (0x1f < (ulonglong)((lStack_f8 + -8) - lVar9)) goto LAB_146bdf280;
            uVar15 = uStack_e0 + 0x28;
          }
          FUN_14e934b00(lVar9,uVar15);
        }
        if (bVar18) {
          FUN_14e933040(auStack_90);
          FUN_14e933040(auStack_a0);
          return true;
        }
        func_0x00014e933af0(auStack_a0);
      }
      FUN_14e933040(auStack_90);
      FUN_14e933040(auStack_a0);
    }
    FUN_14e932660(lVar1 + 0xf8,&uStack_80);
    puVar16 = (undefined8 *)CONCAT71(uStack_7f,uStack_80);
    if (puVar16 == puStack_78) {
      bVar18 = false;
    }
    else {
      do {
        puStack_b0 = puVar16;
        if (0xf < (ulonglong)puVar16[3]) {
          puStack_b0 = (undefined8 *)*puVar16;
        }
        uStack_a8 = puVar16[2];
        FUN_1403e8120(apppppppuStack_d0,&puStack_b0);
        uVar15 = uStack_b8;
        pppppppuVar17 = apppppppuStack_d0[0];
        lVar9 = lVar1 + 0x60;
        if (0xf < *(ulonglong *)(lVar1 + 0x78)) {
          lVar9 = *(longlong *)(lVar1 + 0x60);
        }
        pppppppuVar10 = apppppppuStack_d0;
        if (0xf < uStack_b8) {
          pppppppuVar10 = apppppppuStack_d0[0];
        }
        if (lStack_c0 == *(longlong *)(lVar1 + 0x70)) {
          if (*(longlong *)(lVar1 + 0x70) == 0) {
            bVar18 = true;
          }
          else {
            iVar6 = FUN_14eec62b0(pppppppuVar10,lVar9);
            bVar18 = iVar6 == 0;
          }
        }
        else {
          bVar18 = false;
        }
        if (0xf < uVar15) {
          uVar12 = uVar15 + 1;
          if (0xfff < uVar12) {
            if (0x1f < (ulonglong)((longlong)pppppppuVar17 + (-8 - (longlong)pppppppuVar17[-1]))) {
LAB_146bdf280:
              _invoke_watson(0,0,0,0,0);
              pcVar2 = (code *)swi(3);
              uVar4 = (*pcVar2)();
              return (bool)uVar4;
            }
            uVar12 = uVar15 + 0x28;
            pppppppuVar17 = (undefined8 *******)pppppppuVar17[-1];
          }
          FUN_14e934b00(pppppppuVar17,uVar12);
        }
        puVar16 = puVar16 + 4;
      } while (puVar16 != puStack_78 && bVar18 == false);
    }
    FUN_140075c10(&uStack_80);
    return bVar18;
  }
  puStack_b0 = (undefined8 *)0x146a03851;
  FUN_1403e7e80(apppppppuStack_60,&UNK_14f2c85f0);
  pppppppuVar17 = apppppppuStack_60[0];
  pppppppuVar10 = apppppppuStack_60;
  if (0xf < uStack_48) {
    pppppppuVar10 = apppppppuStack_60[0];
  }
  plVar8 = plVar11;
  if (0xf < *(ulonglong *)(lVar1 + 0x78)) {
    plVar8 = (longlong *)*plVar11;
  }
  if (*(longlong *)(lVar1 + 0x70) == CONCAT44(uStack_4c,uStack_50)) {
    bVar18 = true;
    if (CONCAT44(uStack_4c,uStack_50) != 0) {
      puStack_b0 = (undefined8 *)0x146a0388b;
      iVar6 = FUN_14eec62b0(plVar8,pppppppuVar10);
      if (iVar6 != 0) goto LAB_146a0388f;
    }
  }
  else {
LAB_146a0388f:
    puStack_b0 = (undefined8 *)0x146a038a2;
    FUN_1403e7e80(&uStack_80,&UNK_14f2c8610);
    puVar14 = (undefined1 *)CONCAT71(uStack_7f,uStack_80);
    puVar13 = &uStack_80;
    if (0xf < uStack_68) {
      puVar13 = puVar14;
    }
    if (0xf < *(ulonglong *)(lVar1 + 0x78)) {
      plVar11 = (longlong *)*plVar11;
    }
    if (*(longlong *)(lVar1 + 0x70) == lStack_70) {
      if (lStack_70 == 0) {
        bVar18 = true;
      }
      else {
        puStack_b0 = (undefined8 *)0x146a038da;
        iVar6 = FUN_14eec62b0(plVar11,puVar13);
        bVar18 = iVar6 == 0;
      }
    }
    else {
      bVar18 = false;
    }
    if (0xf < uStack_68) {
      uVar15 = uStack_68 + 1;
      puVar13 = puVar14;
      if (0xfff < uVar15) {
        puVar13 = *(undefined1 **)(puVar14 + -8);
        if ((undefined1 *)0x1f < puVar14 + (-8 - (longlong)puVar13)) goto LAB_146a03979;
        uVar15 = uStack_68 + 0x28;
      }
      puStack_b0 = (undefined8 *)0x146a03932;
      FUN_14e934b00(puVar13,uVar15);
    }
  }
  if (0xf < uStack_48) {
    uVar15 = uStack_48 + 1;
    pppppppuVar10 = pppppppuVar17;
    if (0xfff < uVar15) {
      pppppppuVar10 = (undefined8 *******)pppppppuVar17[-1];
      if (0x1f < (ulonglong)((longlong)pppppppuVar17 + (-8 - (longlong)pppppppuVar10))) {
LAB_146a03979:
        uStack_88 = 0;
        puStack_b0 = (undefined8 *)0x146a03992;
        _invoke_watson(0,0,0,0);
        pcVar2 = (code *)swi(3);
        uVar4 = (*pcVar2)();
        return (bool)uVar4;
      }
      uVar15 = uStack_48 + 0x28;
    }
    puStack_b0 = (undefined8 *)&LAB_146a03968;
    FUN_14e934b00(pppppppuVar10,uVar15);
  }
  return bVar18;
}

