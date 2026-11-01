
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Restarted to delay deadcode elimination for space: ram */

longlong *
FUN_145421920(longlong *******param_1,longlong *param_2,int *param_3,ulonglong param_4,
             ulonglong param_5,ulonglong param_6)

{
  int *piVar1;
  int iVar2;
  char cVar3;
  longlong ****pppplVar4;
  code *pcVar5;
  bool bVar6;
  int iVar7;
  longlong ******pppppplVar8;
  longlong *******ppppppplVar9;
  longlong *******ppppppplVar10;
  longlong *******ppppppplVar11;
  longlong *****ppppplVar12;
  ulonglong uVar13;
  longlong *plVar14;
  byte bVar15;
  undefined4 uVar16;
  ulonglong uVar17;
  longlong ******pppppplVar18;
  longlong *******ppppppplVar19;
  longlong *******ppppppplVar20;
  longlong *******unaff_RDI;
  longlong *****ppppplVar21;
  longlong *******ppppppplVar22;
  longlong *******unaff_R14;
  uint uVar23;
  bool bVar24;
  undefined *puStack_2d8;
  longlong *******ppppppplStack_2d0;
  undefined1 auStack_298 [56];
  longlong *******ppppppplStack_260;
  undefined *puStack_258;
  longlong *******ppppppplStack_250;
  longlong ******pppppplStack_218;
  longlong *******ppppppplStack_210;
  longlong *******ppppppplStack_208;
  longlong *******ppppppplStack_200;
  longlong *******ppppppplStack_1f8;
  longlong *******ppppppplStack_1f0;
  int iStack_1e8;
  longlong *******ppppppplStack_1e0;
  longlong ******pppppplStack_1d8;
  longlong *******ppppppplStack_1d0;
  longlong *******ppppppplStack_1c8;
  longlong ******pppppplStack_1c0;
  longlong ******pppppplStack_1b8;
  uint uStack_1b0;
  int iStack_1a8;
  longlong ******pppppplStack_1a0;
  longlong ******pppppplStack_198;
  longlong *******ppppppplStack_188;
  int *piStack_180;
  longlong *******ppppppplStack_178;
  longlong *******ppppppplStack_170;
  longlong *******ppppppplStack_168;
  longlong *******ppppppplStack_160;
  longlong *******ppppppplStack_158;
  longlong *plStack_150;
  longlong *******ppppppplStack_148;
  longlong *******ppppppplStack_140;
  longlong *******ppppppplStack_138;
  longlong *******ppppppplStack_130;
  longlong ******pppppplStack_128;
  longlong *******ppppppplStack_120;
  longlong *******ppppppplStack_118;
  longlong *******ppppppplStack_110;
  longlong ******pppppplStack_108;
  longlong ******pppppplStack_100;
  longlong ******pppppplStack_f8;
  longlong ******pppppplStack_f0;
  longlong ******pppppplStack_e8;
  longlong *******ppppppplStack_e0;
  undefined8 uStack_d8;
  int iStack_d0;
  int iStack_cc;
  int aiStack_c8 [2];
  longlong *******ppppppplStack_c0;
  longlong ******pppppplStack_b8;
  longlong *******ppppppplStack_b0;
  ulonglong uStack_a8;
  int iStack_a0;
  int iStack_9c;
  int iStack_98;
  int iStack_94;
  uint uStack_90;
  longlong *******ppppppplStack_88;
  longlong *******ppppppplStack_80;
  longlong *******ppppppplStack_78;
  longlong *******ppppppplStack_70;
  longlong *******ppppppplStack_68;
  longlong *******ppppppplStack_60;
  longlong *******ppppppplStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  ppppppplVar11 = (longlong *******)(param_4 & 0xffffffff);
  aiStack_c8[0] = *param_3;
  ppppppplStack_c0 = (longlong *******)0x0;
  pppppplStack_b8 = (longlong ******)0x0;
  ppppppplStack_b0 = (longlong *******)0x0;
  uStack_a8 = 0;
  ppppppplVar10 = *(longlong ********)(param_3 + 6);
  if (*(ulonglong *)(param_3 + 8) < 0x10) {
    ppppppplVar19 = (longlong *******)(param_3 + 2);
  }
  else {
    ppppppplVar19 = *(longlong ********)(param_3 + 2);
  }
  ppppppplStack_60 = param_1;
  if (-1 < (longlong)ppppppplVar10) {
    ppppppplStack_168 = (longlong *******)&ppppppplStack_c0;
    if (ppppppplVar10 < (longlong *******)0x10) {
      uStack_a8 = 0xf;
      ppppppplStack_c0 = (longlong *******)*ppppppplVar19;
      pppppplStack_b8 = ppppppplVar19[1];
      ppppppplStack_b0 = ppppppplVar10;
    }
    else {
      uVar17 = (ulonglong)ppppppplVar10 | 0xf;
      uVar13 = 0x16;
      if (0x16 < uVar17) {
        uVar13 = uVar17;
      }
      if (uVar17 < 0xfff) {
        unaff_R14 = (longlong *******)(uVar13 + 1);
        ppppppplVar9 = (longlong *******)
                       (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,unaff_R14);
        if (ppppppplVar9 == (longlong *******)0x0) goto LAB_145423226;
      }
      else {
        unaff_R14 = (longlong *******)(uVar13 + 0x28);
        pppppplVar8 = (longlong ******)
                      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,unaff_R14);
        if (pppppplVar8 == (longlong ******)0x0) {
          FUN_1409bc820(unaff_R14);
          unaff_R14 = (longlong *******)0x0;
        }
        ppppppplVar9 = (longlong *******)((longlong)pppppplVar8 + 0x27U & 0xffffffffffffffe0);
        ppppppplVar9[-1] = pppppplVar8;
      }
      ppppppplStack_c0 = ppppppplVar9;
      ppppppplStack_b0 = ppppppplVar10;
      uStack_a8 = uVar13;
      FUN_14eec62c0(ppppppplVar9,ppppppplVar19,(longlong)ppppppplVar10 + 1);
    }
    ppppppplVar10 = ppppppplStack_60;
    iStack_a0 = param_3[10];
    iStack_9c = param_3[0xb];
    iStack_98 = param_3[0xc];
    iStack_94 = param_3[0xd];
    uVar23 = (uint)param_4;
    ppppppplStack_210 = ppppppplStack_60 + 0x49;
    uStack_90 = uVar23;
    pppppplVar8 = (longlong ******)FUN_1454776c0(ppppppplStack_210,aiStack_c8);
    plStack_150 = param_2;
    if (pppppplVar8 != ppppppplVar10[0x49]) {
      if (pppppplVar8[0xd] == (longlong *****)0x0) {
        ppppplVar12 = (longlong *****)0x0;
      }
      else {
        LOCK();
        ppppplVar12 = pppppplVar8[0xd] + 1;
        *(int *)ppppplVar12 = *(int *)ppppplVar12 + 1;
        UNLOCK();
        ppppplVar12 = pppppplVar8[0xd];
      }
      ppppplVar21 = pppppplVar8[0xc];
      *param_2 = (longlong)ppppplVar21;
      param_2[1] = (longlong)ppppplVar12;
      if (ppppplVar21 == (longlong *****)0x0) {
LAB_145421b0a:
        if (ppppplVar12 != (longlong *****)0x0) {
          LOCK();
          ppppplVar21 = ppppplVar12 + 1;
          *(int *)ppppplVar21 = *(int *)ppppplVar21 + -1;
          UNLOCK();
          if (*(int *)ppppplVar21 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppplVar12);
            LOCK();
            piVar1 = (int *)((longlong)ppppplVar12 + 0xc);
            *piVar1 = *piVar1 + -1;
            UNLOCK();
            if (*piVar1 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppplVar12);
            }
          }
        }
        goto LAB_145421b3a;
      }
      iVar7 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppplVar21 + 3);
      plVar14 = plStack_150;
      if ((iVar7 != 0) &&
         (iVar7 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppplVar21 + 3),
         plVar14 = plStack_150, iVar7 != 1)) goto LAB_145421b0a;
      goto LAB_145422e01;
    }
    *param_2 = 0;
    param_2[1] = 0;
LAB_145421b3a:
    ppppppplVar19 = (longlong *******)(param_5 & 0xffffffff);
    if ((param_5 >> 0x20 & 1) == 0) {
      ppppppplVar19 = (longlong *******)0x1;
    }
    FUN_1454756c0(ppppppplStack_60 + 0x35,&ppppppplStack_148,param_3);
    unaff_R14 = ppppppplStack_148;
    uVar16 = 0x40;
    if ((int)ppppppplVar19 == 9) {
      uVar16 = 1;
    }
    if ((param_6 >> 0x20 & 1) != 0) {
      uVar16 = (undefined4)param_6;
    }
    ppppppplStack_160 = (longlong *******)CONCAT44(ppppppplStack_160._4_4_,uVar16);
    bVar15 = 4;
    if (uVar23 != 0) {
      bVar15 = uVar23 == 1;
    }
    *(byte *)((longlong)ppppppplStack_148[0xb] + 0x51) =
         *(byte *)((longlong)ppppppplStack_148[0xb] + 0x51) | bVar15;
    unaff_RDI = (longlong *******)
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,0x20);
    if (unaff_RDI == (longlong *******)0x0) goto LAB_145423077;
    while( true ) {
      ppppppplVar10 = ppppppplStack_60;
      ppppppplStack_158 = ppppppplVar19;
      ppppppplVar19 = unaff_RDI + 1;
      unaff_RDI[1] = (longlong ******)0x100000001;
      piStack_180 = (int *)((longlong)unaff_RDI + 0xc);
      *unaff_RDI = (longlong ******)&UNK_14f099a90;
      unaff_RDI[2] = (longlong ******)0x0;
      unaff_RDI[3] = (longlong ******)0x0;
      param_1 = (longlong *******)ppppppplStack_60[5];
      ppppppplStack_70 = unaff_RDI;
      ppppppplStack_188 = ppppppplVar19;
      if (param_1 == (longlong *******)0x0) break;
      ppppppplStack_58 = unaff_RDI + 2;
      iVar7 = *(int *)(param_1 + 1);
      do {
        if (iVar7 == 0) goto LAB_1454231ae;
        LOCK();
        iVar2 = *(int *)(param_1 + 1);
        bVar24 = iVar7 == iVar2;
        if (bVar24) {
          *(int *)(param_1 + 1) = iVar7 + 1;
          iVar2 = iVar7;
        }
        iVar7 = iVar2;
        UNLOCK();
      } while (!bVar24);
      ppppppplVar10 = (longlong *******)ppppppplStack_60[5];
      param_1 = (longlong *******)(ppppppplStack_60[4] + -3);
      if (ppppppplStack_60[4] == (longlong ******)0x0) {
        param_1 = (longlong *******)0x0;
      }
      ppppppplStack_80 = param_1;
      if (ppppppplVar10 == (longlong *******)0x0) {
        ppppppplStack_80 = (longlong *******)0x0;
      }
      else {
        LOCK();
        *(int *)((longlong)ppppppplVar10 + 0xc) = *(int *)((longlong)ppppppplVar10 + 0xc) + 1;
        UNLOCK();
        LOCK();
        ppppppplVar19 = ppppppplVar10 + 1;
        *(int *)ppppppplVar19 = *(int *)ppppppplVar19 + -1;
        UNLOCK();
        if (*(int *)ppppppplVar19 == 0) {
          piVar1 = (int *)((longlong)ppppppplVar10 + 0xc);
          param_1 = ppppppplVar10;
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
          LOCK();
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            param_1 = ppppppplVar10;
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
          }
        }
      }
      ppppppplVar9 = ppppppplStack_b0;
      if (ppppppplStack_60[0x37] == (longlong ******)0x0) {
        _DAT_00000000 = 0xdeadc0de;
      }
      else if (*(char *)ppppppplStack_60[0x37] == '\0') {
        _DAT_00000000 = 0xdeadc0de;
      }
      pppppplStack_218 = ppppppplStack_60[0x39];
      ppppppplStack_148 = (longlong *******)0x0;
      ppppppplStack_140 = (longlong *******)0x0;
      if (ppppppplVar10 != (longlong *******)0x0) {
        ppppppplStack_148 = ppppppplStack_80;
        ppppppplStack_140 = ppppppplVar10;
        LOCK();
        *(int *)((longlong)ppppppplVar10 + 0xc) = *(int *)((longlong)ppppppplVar10 + 0xc) + 1;
        UNLOCK();
      }
      ppppppplStack_88 = ppppppplVar10;
      LOCK();
      *(int *)(unaff_RDI + 1) = *(int *)(unaff_RDI + 1) + 1;
      UNLOCK();
      ppppppplStack_138 = ppppppplStack_58;
      ppppppplStack_130 = unaff_RDI;
      pppppplStack_128 = (longlong ******)CONCAT44(pppppplStack_128._4_4_,aiStack_c8[0]);
      ppppppplStack_120 = (longlong *******)0x0;
      ppppppplStack_118 = (longlong *******)0x0;
      ppppppplStack_110 = (longlong *******)0x0;
      pppppplStack_108 = (longlong ******)0x0;
      ppppppplVar19 = (longlong *******)&ppppppplStack_c0;
      if (0xf < uStack_a8) {
        ppppppplVar19 = ppppppplStack_c0;
      }
      ppppppplVar10 = ppppppplStack_60;
      if ((longlong)ppppppplStack_b0 < 0) goto LAB_1454231ba;
      if (ppppppplStack_b0 < (longlong *******)0x10) {
        ppppppplStack_110 = ppppppplStack_b0;
        pppppplStack_108 = (longlong ******)0xf;
        ppppppplStack_120 = (longlong *******)*ppppppplVar19;
        ppppppplStack_118 = (longlong *******)ppppppplVar19[1];
      }
      else {
        pppppplVar18 = (longlong ******)((ulonglong)ppppppplStack_b0 | 0xf);
        pppppplVar8 = (longlong ******)0x16;
        if ((longlong ******)0x16 < pppppplVar18) {
          pppppplVar8 = pppppplVar18;
        }
        if (pppppplVar18 < (longlong ******)0xfff) {
          ppppppplVar10 =
               (longlong *******)
               (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                         (_DAT_15233c1a8,(longlong)pppppplVar8 + 1);
          if (ppppppplVar10 == (longlong *******)0x0) {
            FUN_1409bc820((longlong)pppppplVar8 + 1);
            ppppppplVar10 = (longlong *******)0x0;
          }
        }
        else {
          pppppplVar18 = (longlong ******)
                         (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                                   (_DAT_15233c1a8,pppppplVar8 + 5);
          if (pppppplVar18 == (longlong ******)0x0) {
            FUN_1409bc820(pppppplVar8 + 5);
          }
          ppppppplVar10 = (longlong *******)((longlong)pppppplVar18 + 0x27U & 0xffffffffffffffe0);
          ppppppplVar10[-1] = pppppplVar18;
        }
        ppppppplStack_110 = ppppppplVar9;
        ppppppplStack_120 = ppppppplVar10;
        pppppplStack_108 = pppppplVar8;
        FUN_14eec62c0(ppppppplVar10,ppppppplVar19,(longlong)ppppppplVar9 + 1);
      }
      ppppppplVar10 = ppppppplStack_60;
      pppppplStack_100 = (longlong ******)CONCAT44(iStack_9c,iStack_a0);
      pppppplStack_f8 = (longlong ******)CONCAT44(iStack_94,iStack_98);
      pppppplStack_f0 = (longlong ******)CONCAT44(pppppplStack_f0._4_4_,uStack_90);
      if (ppppppplStack_60[0x4e] == (longlong ******)0x0) {
        ppppppplStack_e0 = (longlong *******)0x0;
      }
      else {
        LOCK();
        pppppplVar8 = ppppppplStack_60[0x4e] + 1;
        *(int *)pppppplVar8 = *(int *)pppppplVar8 + 1;
        UNLOCK();
        ppppppplStack_e0 = (longlong *******)ppppppplStack_60[0x4e];
      }
      pppppplStack_e8 = ppppppplStack_60[0x4d];
      uStack_d8 = (longlong *******)((longlong)unaff_R14[0xb] + 0x51);
      ppppppplStack_250 = (longlong *******)0x0;
      puStack_258 = &UNK_14f24c3e8;
      iStack_d0 = (int)ppppppplVar11;
      ppppppplVar9 = (longlong *******)
                     (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,0x80);
      if (ppppppplVar9 == (longlong *******)0x0) {
        FUN_1409bc820();
      }
      ppppppplVar20 = ppppppplStack_110;
      *ppppppplVar9 = (longlong ******)0x0;
      ppppppplVar9[1] = (longlong ******)0x0;
      if (ppppppplStack_140 != (longlong *******)0x0) {
        *ppppppplVar9 = (longlong ******)ppppppplStack_148;
        ppppppplVar9[1] = (longlong ******)ppppppplStack_140;
        LOCK();
        *(int *)((longlong)ppppppplStack_140 + 0xc) =
             *(int *)((longlong)ppppppplStack_140 + 0xc) + 1;
        UNLOCK();
      }
      ppppppplVar9[2] = (longlong ******)0x0;
      ppppppplVar9[3] = (longlong ******)0x0;
      if (ppppppplStack_130 == (longlong *******)0x0) {
        param_1 = (longlong *******)0x0;
      }
      else {
        LOCK();
        *(int *)(ppppppplStack_130 + 1) = *(int *)(ppppppplStack_130 + 1) + 1;
        UNLOCK();
        param_1 = ppppppplStack_130;
      }
      ppppppplVar9[2] = (longlong ******)ppppppplStack_138;
      ppppppplVar9[3] = (longlong ******)param_1;
      *(int *)(ppppppplVar9 + 4) = (int)pppppplStack_128;
      ppppppplVar9[5] = (longlong ******)0x0;
      ppppppplVar9[6] = (longlong ******)0x0;
      ppppppplVar9[7] = (longlong ******)0x0;
      ppppppplVar9[8] = (longlong ******)0x0;
      ppppppplVar22 = (longlong *******)&ppppppplStack_120;
      if ((longlong ******)0xf < pppppplStack_108) {
        ppppppplVar22 = ppppppplStack_120;
      }
      ppppppplVar19 = ppppppplVar9;
      unaff_RDI = ppppppplVar9;
      if ((longlong)ppppppplStack_110 < 0) goto LAB_1454231c0;
      if (ppppppplStack_110 < (longlong *******)0x10) {
        ppppppplVar9[7] = (longlong ******)ppppppplStack_110;
        ppppppplVar9[8] = (longlong ******)0xf;
        pppppplVar8 = ppppppplVar22[1];
        ppppppplVar9[5] = *ppppppplVar22;
        ppppppplVar9[6] = pppppplVar8;
      }
      else {
        ppppppplVar10 = (longlong *******)((ulonglong)ppppppplStack_110 | 0xf);
        unaff_RDI = (longlong *******)0x16;
        if ((longlong *******)0x16 < ppppppplVar10) {
          unaff_RDI = ppppppplVar10;
        }
        ppppppplStack_78 = param_1;
        ppppppplStack_68 = ppppppplVar9;
        if (ppppppplVar10 < (longlong *******)0xfff) {
          pppppplVar8 = (longlong ******)
                        (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                                  (_DAT_15233c1a8,(longlong)unaff_RDI + 1);
          if (pppppplVar8 == (longlong ******)0x0) {
            FUN_1409bc820((longlong)unaff_RDI + 1);
            pppppplVar8 = (longlong ******)0x0;
          }
        }
        else {
          ppppplVar12 = (longlong *****)
                        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,unaff_RDI + 5)
          ;
          if (ppppplVar12 == (longlong *****)0x0) {
            FUN_1409bc820(unaff_RDI + 5);
          }
          pppppplVar8 = (longlong ******)((longlong)ppppplVar12 + 0x27U & 0xffffffffffffffe0);
          pppppplVar8[-1] = ppppplVar12;
        }
        ppppppplVar9 = ppppppplStack_68;
        ppppppplStack_68[5] = pppppplVar8;
        ppppppplStack_68[7] = (longlong ******)ppppppplVar20;
        ppppppplStack_68[8] = (longlong ******)unaff_RDI;
        FUN_14eec62c0(pppppplVar8,ppppppplVar22,(longlong)ppppppplVar20 + 1);
        ppppppplVar10 = ppppppplStack_60;
      }
      ppppppplVar20 = ppppppplStack_b0;
      ppppppplVar9[9] = pppppplStack_100;
      ppppppplVar9[10] = pppppplStack_f8;
      *(int *)(ppppppplVar9 + 0xb) = (int)pppppplStack_f0;
      ppppppplVar9[0xc] = pppppplStack_e8;
      ppppppplVar9[0xd] = (longlong ******)ppppppplStack_e0;
      pppppplStack_e8 = (longlong ******)0x0;
      ppppppplStack_e0 = (longlong *******)0x0;
      *(int *)(ppppppplVar9 + 0xe) = (int)uStack_d8;
      *(int *)((longlong)ppppppplVar9 + 0x74) = uStack_d8._4_4_;
      *(int *)(ppppppplVar9 + 0xf) = iStack_d0;
      *(int *)((longlong)ppppppplVar9 + 0x7c) = iStack_cc;
      ppppppplStack_208 = (longlong *******)0x0;
      ppppppplStack_200 = (longlong *******)0x0;
      if (ppppppplStack_88 != (longlong *******)0x0) {
        ppppppplStack_208 = ppppppplStack_80;
        ppppppplStack_200 = ppppppplStack_88;
        LOCK();
        *(int *)((longlong)ppppppplStack_88 + 0xc) = *(int *)((longlong)ppppppplStack_88 + 0xc) + 1;
        UNLOCK();
      }
      LOCK();
      *(int *)(ppppppplStack_70 + 1) = *(int *)(ppppppplStack_70 + 1) + 1;
      UNLOCK();
      ppppppplStack_1f8 = ppppppplStack_58;
      ppppppplStack_1f0 = ppppppplStack_70;
      iStack_1e8 = aiStack_c8[0];
      ppppppplStack_1e0 = (longlong *******)0x0;
      pppppplStack_1d8 = (longlong ******)0x0;
      ppppppplStack_1d0 = (longlong *******)0x0;
      ppppppplStack_1c8 = (longlong *******)0x0;
      ppppppplVar19 = (longlong *******)&ppppppplStack_c0;
      if (0xf < uStack_a8) {
        ppppppplVar19 = ppppppplStack_c0;
      }
      ppppppplStack_250 = ppppppplVar9;
      if ((longlong)ppppppplStack_b0 < 0) goto LAB_1454231d4;
      if (ppppppplStack_b0 < (longlong *******)0x10) {
        ppppppplStack_1d0 = ppppppplStack_b0;
        ppppppplStack_1c8 = (longlong *******)0xf;
        ppppppplStack_1e0 = (longlong *******)*ppppppplVar19;
        pppppplStack_1d8 = ppppppplVar19[1];
      }
      else {
        ppppppplVar9 = (longlong *******)((ulonglong)ppppppplStack_b0 | 0xf);
        ppppppplVar10 = (longlong *******)0x16;
        if ((longlong *******)0x16 < ppppppplVar9) {
          ppppppplVar10 = ppppppplVar9;
        }
        if (ppppppplVar9 < (longlong *******)0xfff) {
          ppppppplVar9 = (longlong *******)
                         (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                                   (_DAT_15233c1a8,(longlong)ppppppplVar10 + 1);
          if (ppppppplVar9 == (longlong *******)0x0) {
            FUN_1409bc820((longlong)ppppppplVar10 + 1);
            ppppppplVar9 = (longlong *******)0x0;
          }
        }
        else {
          pppppplVar8 = (longlong ******)
                        (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                                  (_DAT_15233c1a8,ppppppplVar10 + 5);
          if (pppppplVar8 == (longlong ******)0x0) {
            FUN_1409bc820(ppppppplVar10 + 5);
          }
          ppppppplVar9 = (longlong *******)((longlong)pppppplVar8 + 0x27U & 0xffffffffffffffe0);
          ppppppplVar9[-1] = pppppplVar8;
        }
        ppppppplStack_1d0 = ppppppplVar20;
        ppppppplStack_1e0 = ppppppplVar9;
        ppppppplStack_1c8 = ppppppplVar10;
        FUN_14eec62c0(ppppppplVar9,ppppppplVar19,(longlong)ppppppplVar20 + 1);
        ppppppplVar10 = ppppppplStack_60;
      }
      pppppplStack_1c0 = (longlong ******)CONCAT44(iStack_9c,iStack_a0);
      pppppplStack_1b8 = (longlong ******)CONCAT44(iStack_94,iStack_98);
      uStack_1b0 = uStack_90;
      if (ppppppplVar10[0x4e] == (longlong ******)0x0) {
        pppppplStack_198 = (longlong ******)0x0;
      }
      else {
        LOCK();
        pppppplVar8 = ppppppplVar10[0x4e] + 1;
        *(int *)pppppplVar8 = *(int *)pppppplVar8 + 1;
        UNLOCK();
        pppppplStack_198 = ppppppplVar10[0x4e];
      }
      pppppplStack_1a0 = ppppppplVar10[0x4d];
      ppppppplStack_2d0 = (longlong *******)0x0;
      puStack_2d8 = &UNK_14f24c3d0;
      iStack_1a8 = (int)ppppppplVar11;
      ppppppplVar10 =
           (longlong *******)(*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,0x78);
      if (ppppppplVar10 == (longlong *******)0x0) {
        FUN_1409bc820(0x78);
      }
      unaff_R14 = ppppppplStack_1c8;
      ppppppplVar19 = ppppppplStack_1d0;
      ppppppplVar11 = ppppppplStack_1e0;
      *ppppppplVar10 = (longlong ******)0x0;
      ppppppplVar10[1] = (longlong ******)0x0;
      if (ppppppplStack_200 != (longlong *******)0x0) {
        *ppppppplVar10 = (longlong ******)ppppppplStack_208;
        ppppppplVar10[1] = (longlong ******)ppppppplStack_200;
        LOCK();
        *(int *)((longlong)ppppppplStack_200 + 0xc) =
             *(int *)((longlong)ppppppplStack_200 + 0xc) + 1;
        UNLOCK();
      }
      ppppppplVar10[2] = (longlong ******)0x0;
      ppppppplVar10[3] = (longlong ******)0x0;
      if (ppppppplStack_1f0 == (longlong *******)0x0) {
        unaff_RDI = (longlong *******)0x0;
      }
      else {
        LOCK();
        *(int *)(ppppppplStack_1f0 + 1) = *(int *)(ppppppplStack_1f0 + 1) + 1;
        UNLOCK();
        unaff_RDI = ppppppplStack_1f0;
      }
      ppppppplVar10[2] = (longlong ******)ppppppplStack_1f8;
      ppppppplVar10[3] = (longlong ******)unaff_RDI;
      *(int *)(ppppppplVar10 + 4) = iStack_1e8;
      ppppppplVar10[5] = (longlong ******)0x0;
      ppppppplVar10[6] = (longlong ******)0x0;
      ppppppplVar10[7] = (longlong ******)0x0;
      ppppppplVar10[8] = (longlong ******)0x0;
      ppppppplVar9 = (longlong *******)&ppppppplStack_1e0;
      if ((longlong *******)0xf < ppppppplStack_1c8) {
        ppppppplVar9 = ppppppplStack_1e0;
      }
      if ((longlong)ppppppplStack_1d0 < 0) goto LAB_1454231da;
      if (ppppppplStack_1d0 < (longlong *******)0x10) {
        ppppppplVar10[7] = (longlong ******)ppppppplStack_1d0;
        ppppppplVar10[8] = (longlong ******)0xf;
        pppppplVar8 = ppppppplVar9[1];
        ppppppplVar10[5] = *ppppppplVar9;
        ppppppplVar10[6] = pppppplVar8;
      }
      else {
        pppppplVar18 = (longlong ******)((ulonglong)ppppppplStack_1d0 | 0xf);
        pppppplVar8 = (longlong ******)0x16;
        if ((longlong ******)0x16 < pppppplVar18) {
          pppppplVar8 = pppppplVar18;
        }
        ppppppplStack_78 = unaff_RDI;
        ppppppplStack_68 = ppppppplVar10;
        if (pppppplVar18 < (longlong ******)0xfff) {
          pppppplVar18 = (longlong ******)
                         (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                                   (_DAT_15233c1a8,(longlong)pppppplVar8 + 1U);
          if (pppppplVar18 == (longlong ******)0x0) {
            FUN_1409bc820((longlong)pppppplVar8 + 1U);
            pppppplVar18 = (longlong ******)0x0;
          }
        }
        else {
          ppppplVar12 = (longlong *****)
                        (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                                  (_DAT_15233c1a8,pppppplVar8 + 5);
          if (ppppplVar12 == (longlong *****)0x0) {
            FUN_1409bc820(pppppplVar8 + 5);
          }
          pppppplVar18 = (longlong ******)((longlong)ppppplVar12 + 0x27U & 0xffffffffffffffe0);
          pppppplVar18[-1] = ppppplVar12;
        }
        ppppppplVar10 = ppppppplStack_68;
        ppppppplStack_68[5] = pppppplVar18;
        ppppppplStack_68[7] = (longlong ******)ppppppplVar19;
        ppppppplStack_68[8] = pppppplVar8;
        FUN_14eec62c0(pppppplVar18,ppppppplVar9,(longlong)ppppppplVar19 + 1);
        unaff_RDI = ppppppplStack_78;
      }
      ppppppplVar10[9] = pppppplStack_1c0;
      ppppppplVar10[10] = pppppplStack_1b8;
      *(uint *)(ppppppplVar10 + 0xb) = uStack_1b0;
      *(int *)(ppppppplVar10 + 0xc) = iStack_1a8;
      ppppppplVar10[0xd] = pppppplStack_1a0;
      ppppppplVar10[0xe] = pppppplStack_198;
      pppppplStack_1a0 = (longlong ******)0x0;
      pppppplStack_198 = (longlong ******)0x0;
      uStack_49 = 0;
      ppppppplStack_2d0 = ppppppplVar10;
      FUN_141871a10(pppppplStack_218,&ppppppplStack_178,ppppppplStack_158,&puStack_2d8,&puStack_258,
                    ppppppplStack_160._0_4_);
      ppppppplVar10 = ppppppplStack_88;
      if ((longlong *******)0xf < unaff_R14) {
        ppppppplVar19 = (longlong *******)((longlong)unaff_R14 + 1);
        ppppppplVar9 = ppppppplVar11;
        if ((longlong *******)0xfff < ppppppplVar19) {
          ppppppplVar9 = (longlong *******)ppppppplVar11[-1];
          if (0x1f < (ulonglong)((longlong)ppppppplVar11 + (-8 - (longlong)ppppppplVar9)))
          goto LAB_14542320d;
          ppppppplVar19 = unaff_R14 + 5;
        }
        FUN_14e934b00(ppppppplVar9,ppppppplVar19);
      }
      ppppppplStack_1d0 = (longlong *******)0x0;
      ppppppplStack_1c8 = (longlong *******)0xf;
      ppppppplStack_1e0 = (longlong *******)((ulonglong)ppppppplStack_1e0 & 0xffffffffffffff00);
      if (unaff_RDI != (longlong *******)0x0) {
        LOCK();
        ppppppplVar11 = unaff_RDI + 1;
        *(int *)ppppppplVar11 = *(int *)ppppppplVar11 + -1;
        UNLOCK();
        if (*(int *)ppppppplVar11 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(unaff_RDI);
          LOCK();
          piVar1 = (int *)((longlong)unaff_RDI + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(unaff_RDI);
          }
        }
      }
      if (ppppppplStack_200 != (longlong *******)0x0) {
        LOCK();
        piVar1 = (int *)((longlong)ppppppplStack_200 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
        }
      }
      FUN_145423c90(&ppppppplStack_148);
      ppppppplVar9 = ppppppplStack_b0;
      unaff_R14 = ppppppplStack_178;
      ppppppplStack_148 = (longlong *******)0x0;
      ppppppplStack_140 = (longlong *******)0x0;
      if (ppppppplVar10 != (longlong *******)0x0) {
        ppppppplStack_148 = ppppppplStack_80;
        ppppppplStack_140 = ppppppplVar10;
        LOCK();
        *(int *)((longlong)ppppppplVar10 + 0xc) = *(int *)((longlong)ppppppplVar10 + 0xc) + 1;
        UNLOCK();
      }
      ppppppplStack_138 = (longlong *******)CONCAT44(ppppppplStack_138._4_4_,aiStack_c8[0]);
      ppppppplStack_130 = (longlong *******)0x0;
      pppppplStack_128 = (longlong ******)0x0;
      ppppppplStack_120 = (longlong *******)0x0;
      ppppppplStack_118 = (longlong *******)0x0;
      ppppppplVar19 = (longlong *******)&ppppppplStack_c0;
      if (0xf < uStack_a8) {
        ppppppplVar19 = ppppppplStack_c0;
      }
      if ((longlong)ppppppplStack_b0 < 0) goto LAB_1454231ee;
      ppppppplVar11 = (longlong *******)&ppppppplStack_130;
      if (ppppppplStack_b0 < (longlong *******)0x10) {
        ppppppplStack_120 = ppppppplStack_b0;
        ppppppplStack_118 = (longlong *******)0xf;
        ppppppplStack_130 = (longlong *******)*ppppppplVar19;
        pppppplStack_128 = ppppppplVar19[1];
      }
      else {
        ppppppplVar10 = (longlong *******)((ulonglong)ppppppplStack_b0 | 0xf);
        unaff_RDI = (longlong *******)0x16;
        if ((longlong *******)0x16 < ppppppplVar10) {
          unaff_RDI = ppppppplVar10;
        }
        if (ppppppplVar10 < (longlong *******)0xfff) {
          ppppppplVar10 =
               (longlong *******)
               (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                         (_DAT_15233c1a8,(longlong)unaff_RDI + 1);
          if (ppppppplVar10 == (longlong *******)0x0) {
            FUN_1409bc820((longlong)unaff_RDI + 1);
            ppppppplVar10 = (longlong *******)0x0;
          }
        }
        else {
          pppppplVar8 = (longlong ******)
                        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,unaff_RDI + 5)
          ;
          if (pppppplVar8 == (longlong ******)0x0) {
            FUN_1409bc820(unaff_RDI + 5);
          }
          ppppppplVar10 = (longlong *******)((longlong)pppppplVar8 + 0x27U & 0xffffffffffffffe0);
          ppppppplVar10[-1] = pppppplVar8;
        }
        ppppppplStack_120 = ppppppplVar9;
        ppppppplStack_130 = ppppppplVar10;
        ppppppplStack_118 = unaff_RDI;
        FUN_14eec62c0(ppppppplVar10,ppppppplVar19,(longlong)ppppppplVar9 + 1);
      }
      ppppppplVar10 = ppppppplStack_60;
      ppppppplStack_110 = (longlong *******)CONCAT44(iStack_9c,iStack_a0);
      pppppplStack_108 = (longlong ******)CONCAT44(iStack_94,iStack_98);
      pppppplStack_100 = (longlong ******)CONCAT44(pppppplStack_100._4_4_,uStack_90);
      if (ppppppplStack_60[0x38] == (longlong ******)0x0) {
        pppppplStack_f0 = (longlong ******)0x0;
      }
      else {
        LOCK();
        pppppplVar8 = ppppppplStack_60[0x38] + 1;
        *(int *)pppppplVar8 = *(int *)pppppplVar8 + 1;
        UNLOCK();
        pppppplStack_f0 = ppppppplStack_60[0x38];
      }
      pppppplStack_f8 = ppppppplStack_60[0x37];
      pppppplStack_e8 = ppppppplStack_60[0x39];
      if ((pppppplStack_f8 != (longlong ******)0x0) && (*(char *)pppppplStack_f8 != '\0')) {
        LOCK();
        *(int *)(ppppppplStack_70 + 1) = *(int *)(ppppppplStack_70 + 1) + 1;
        UNLOCK();
        ppppppplStack_e0 = ppppppplStack_58;
        uStack_d8 = ppppppplStack_70;
        ppppppplStack_260 = (longlong *******)0x0;
        ppppppplVar19 =
             (longlong *******)(*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,0x80);
        if (ppppppplVar19 == (longlong *******)0x0) {
          FUN_1409bc820(0x80);
        }
        ppppppplVar9 = ppppppplStack_120;
        *ppppppplVar19 = (longlong ******)&UNK_14f24c400;
        ppppppplVar19[1] = (longlong ******)0x0;
        ppppppplVar19[2] = (longlong ******)0x0;
        if (ppppppplStack_140 != (longlong *******)0x0) {
          ppppppplVar19[1] = (longlong ******)ppppppplStack_148;
          ppppppplVar19[2] = (longlong ******)ppppppplStack_140;
          LOCK();
          *(int *)((longlong)ppppppplStack_140 + 0xc) =
               *(int *)((longlong)ppppppplStack_140 + 0xc) + 1;
          UNLOCK();
        }
        *(int *)(ppppppplVar19 + 3) = (int)ppppppplStack_138;
        ppppppplVar19[4] = (longlong ******)0x0;
        ppppppplVar19[5] = (longlong ******)0x0;
        ppppppplVar19[6] = (longlong ******)0x0;
        ppppppplVar19[7] = (longlong ******)0x0;
        if ((longlong *******)0xf < ppppppplStack_118) {
          ppppppplVar11 = ppppppplStack_130;
        }
        if ((longlong)ppppppplStack_120 < 0) goto LAB_1454231f4;
        if (ppppppplStack_120 < (longlong *******)0x10) {
          ppppppplVar19[6] = (longlong ******)ppppppplStack_120;
          ppppppplVar19[7] = (longlong ******)0xf;
          pppppplVar8 = ppppppplVar11[1];
          ppppppplVar19[4] = *ppppppplVar11;
          ppppppplVar19[5] = pppppplVar8;
        }
        else {
          ppppppplVar10 = (longlong *******)((ulonglong)ppppppplStack_120 | 0xf);
          unaff_RDI = (longlong *******)0x16;
          if ((longlong *******)0x16 < ppppppplVar10) {
            unaff_RDI = ppppppplVar10;
          }
          ppppppplStack_58 = ppppppplVar19;
          if (ppppppplVar10 < (longlong *******)0xfff) {
            pppppplVar8 = (longlong ******)
                          (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                                    (_DAT_15233c1a8,(longlong)unaff_RDI + 1);
            if (pppppplVar8 == (longlong ******)0x0) {
              FUN_1409bc820((longlong)unaff_RDI + 1);
              pppppplVar8 = (longlong ******)0x0;
            }
          }
          else {
            ppppplVar12 = (longlong *****)
                          (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                                    (_DAT_15233c1a8,unaff_RDI + 5);
            if (ppppplVar12 == (longlong *****)0x0) {
              FUN_1409bc820(unaff_RDI + 5);
            }
            pppppplVar8 = (longlong ******)((longlong)ppppplVar12 + 0x27U & 0xffffffffffffffe0);
            pppppplVar8[-1] = ppppplVar12;
          }
          ppppppplVar19 = ppppppplStack_58;
          ppppppplStack_58[4] = pppppplVar8;
          ppppppplStack_58[6] = (longlong ******)ppppppplVar9;
          ppppppplStack_58[7] = (longlong ******)unaff_RDI;
          FUN_14eec62c0(pppppplVar8,ppppppplVar11,(longlong)ppppppplVar9 + 1);
          ppppppplVar10 = ppppppplStack_60;
        }
        ppppppplVar19[8] = (longlong ******)ppppppplStack_110;
        ppppppplVar19[9] = pppppplStack_108;
        *(int *)(ppppppplVar19 + 10) = (int)pppppplStack_100;
        ppppppplVar19[0xb] = (longlong ******)0x0;
        ppppppplVar19[0xc] = (longlong ******)0x0;
        if (pppppplStack_f0 == (longlong ******)0x0) {
          pppppplVar8 = (longlong ******)0x0;
        }
        else {
          LOCK();
          *(int *)(pppppplStack_f0 + 1) = *(int *)(pppppplStack_f0 + 1) + 1;
          UNLOCK();
          pppppplVar8 = pppppplStack_f0;
        }
        ppppppplVar19[0xb] = pppppplStack_f8;
        ppppppplVar19[0xc] = pppppplVar8;
        ppppppplVar19[0xd] = pppppplStack_e8;
        if ((pppppplStack_f8 != (longlong ******)0x0) && (*(char *)pppppplStack_f8 != '\0')) {
          ppppppplVar19[0xe] = (longlong ******)0x0;
          ppppppplVar19[0xf] = (longlong ******)0x0;
          if (uStack_d8 == (longlong *******)0x0) {
            ppppppplVar11 = (longlong *******)0x0;
          }
          else {
            LOCK();
            *(int *)(uStack_d8 + 1) = *(int *)(uStack_d8 + 1) + 1;
            UNLOCK();
            ppppppplVar11 = uStack_d8;
          }
          ppppppplVar19[0xe] = (longlong ******)ppppppplStack_e0;
          ppppppplVar19[0xf] = (longlong ******)ppppppplVar11;
          ppppppplStack_260 = ppppppplVar19;
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(unaff_R14,auStack_298);
          FUN_145423dc0(&ppppppplStack_148);
          uVar23 = uStack_90;
          ppppppplVar19 = ppppppplStack_b0;
          ppppppplStack_140 = ppppppplStack_170;
          ppppppplStack_148 = ppppppplStack_178;
          ppppppplStack_178 = (longlong *******)0x0;
          ppppppplStack_170 = (longlong *******)0x0;
          ppppppplStack_158 = (longlong *******)ppppppplVar10[0x49];
          ppppppplVar11 = (longlong *******)ppppppplStack_158[1];
          bVar24 = true;
          ppppppplStack_80 = ppppppplStack_140;
          ppppppplStack_160 = ppppppplStack_148;
          ppppppplStack_58 = ppppppplStack_158;
          if (*(char *)((longlong)ppppppplVar11 + 0x19) != '\0') goto LAB_145422947;
          unaff_R14 = (longlong *******)&ppppppplStack_c0;
          ppppppplVar10 = ppppppplVar11;
          if (0xf < uStack_a8) {
            unaff_R14 = ppppppplStack_c0;
          }
          goto LAB_1454229e5;
        }
      }
      FUN_140078ed0();
LAB_145423077:
      FUN_1409bc820(0x20);
    }
LAB_1454231ae:
    FUN_1400a4710();
  }
  FUN_140075720();
LAB_1454231ba:
  FUN_140075720();
LAB_1454231c0:
  ppppppplStack_78 = param_1;
  ppppppplStack_68 = ppppppplVar19;
  FUN_140075720();
LAB_1454231d4:
  FUN_140075720();
LAB_1454231da:
  ppppppplStack_78 = unaff_RDI;
  ppppppplStack_68 = ppppppplVar10;
  FUN_140075720();
LAB_1454231ee:
  FUN_140075720();
LAB_1454231f4:
  ppppppplStack_58 = ppppppplVar19;
  FUN_140075720();
LAB_145423201:
  FUN_1400a4ea0();
LAB_145423207:
  FUN_140075720();
  goto LAB_14542320d;
LAB_1454229e5:
  do {
    ppppppplVar11 = ppppppplVar10;
    if (ppppppplVar11[8] < (longlong ******)0x10) {
      ppppppplVar10 = ppppppplVar11 + 5;
    }
    else {
      ppppppplVar10 = (longlong *******)ppppppplVar11[5];
    }
    ppppppplVar9 = (longlong *******)ppppppplVar11[7];
    ppppppplVar20 = ppppppplVar9;
    if (ppppppplVar19 < ppppppplVar9) {
      ppppppplVar20 = ppppppplVar19;
    }
    iVar7 = FUN_14eec62b0(ppppppplVar10,unaff_R14,ppppppplVar20);
    bVar24 = ppppppplVar9 < ppppppplVar19;
    if (iVar7 != 0) {
      bVar24 = iVar7 < 0;
    }
    if (bVar24) {
LAB_1454229d0:
      ppppppplVar9 = ppppppplVar11 + 2;
      bVar24 = true;
    }
    else {
      iVar7 = FUN_14eec62b0(unaff_R14,ppppppplVar10,ppppppplVar20);
      bVar24 = ppppppplVar9 <= ppppppplVar19;
      if (iVar7 != 0) {
        bVar24 = -1 < iVar7;
      }
      if ((bVar24) && (*(uint *)(ppppppplVar11 + 0xb) < uVar23)) goto LAB_1454229d0;
      bVar24 = false;
      ppppppplVar9 = ppppppplVar11;
      ppppppplStack_58 = ppppppplVar11;
    }
    ppppppplVar10 = (longlong *******)*ppppppplVar9;
  } while (*(char *)((longlong)*ppppppplVar9 + 0x19) == '\0');
LAB_145422947:
  ppppppplVar10 = ppppppplStack_b0;
  if (*(char *)((longlong)ppppppplStack_58 + 0x19) != '\0') {
LAB_145422ab0:
    ppppppplVar10 = ppppppplStack_60;
    if (ppppppplStack_60[0x4a] == (longlong ******)0x249249249249249) goto LAB_145423201;
    ppppppplVar19 =
         (longlong *******)(*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,0x70);
    if (ppppppplVar19 == (longlong *******)0x0) {
      FUN_1409bc820(0x70);
    }
    ppppppplVar9 = ppppppplStack_b0;
    *(int *)(ppppppplVar19 + 4) = aiStack_c8[0];
    ppppppplVar19[5] = (longlong ******)0x0;
    ppppppplVar19[6] = (longlong ******)0x0;
    ppppppplVar19[7] = (longlong ******)0x0;
    ppppppplVar19[8] = (longlong ******)0x0;
    if (0xf < uStack_a8) {
      ppppppplStack_168 = ppppppplStack_c0;
    }
    unaff_R14 = ppppppplStack_b0;
    ppppppplStack_58 = ppppppplVar19;
    if (-1 < (longlong)ppppppplStack_b0) {
      if (ppppppplStack_b0 < (longlong *******)0x10) {
        ppppppplVar19[7] = (longlong ******)ppppppplStack_b0;
        ppppppplVar19[8] = (longlong ******)0xf;
        pppppplVar8 = ppppppplStack_168[1];
        ppppppplVar19[5] = *ppppppplStack_168;
        ppppppplVar19[6] = pppppplVar8;
      }
      else {
        pppppplVar18 = (longlong ******)((ulonglong)ppppppplStack_b0 | 0xf);
        pppppplVar8 = (longlong ******)0x16;
        if ((longlong ******)0x16 < pppppplVar18) {
          pppppplVar8 = pppppplVar18;
        }
        if (pppppplVar18 < (longlong ******)0xfff) {
          pppppplVar18 = (longlong ******)
                         (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                                   (_DAT_15233c1a8,(longlong)pppppplVar8 + 1U);
          if (pppppplVar18 == (longlong ******)0x0) {
            FUN_1409bc820((longlong)pppppplVar8 + 1U);
            pppppplVar18 = (longlong ******)0x0;
          }
        }
        else {
          ppppplVar12 = (longlong *****)
                        (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                                  (_DAT_15233c1a8,pppppplVar8 + 5);
          if (ppppplVar12 == (longlong *****)0x0) {
            FUN_1409bc820(pppppplVar8 + 5);
          }
          pppppplVar18 = (longlong ******)((longlong)ppppplVar12 + 0x27U & 0xffffffffffffffe0);
          pppppplVar18[-1] = ppppplVar12;
        }
        ppppppplVar19 = ppppppplStack_58;
        ppppppplStack_58[5] = pppppplVar18;
        ppppppplStack_58[7] = (longlong ******)ppppppplVar9;
        ppppppplStack_58[8] = pppppplVar8;
        FUN_14eec62c0(pppppplVar18,ppppppplStack_168,(longlong)ppppppplVar9 + 1);
        ppppppplVar10 = ppppppplStack_60;
      }
      ppppppplVar19[9] = (longlong ******)CONCAT44(iStack_9c,iStack_a0);
      ppppppplVar19[10] = (longlong ******)CONCAT44(iStack_94,iStack_98);
      *(uint *)(ppppppplVar19 + 0xb) = uStack_90;
      ppppppplVar19[0xc] = (longlong ******)0x0;
      ppppppplVar19[0xd] = (longlong ******)0x0;
      *ppppppplVar19 = (longlong ******)ppppppplStack_158;
      ppppppplVar19[1] = (longlong ******)ppppppplStack_158;
      ppppppplVar19[2] = (longlong ******)ppppppplStack_158;
      *(undefined2 *)(ppppppplVar19 + 3) = 0;
      ppppppplVar10[0x4a] = (longlong ******)((longlong)ppppppplVar10[0x4a] + 1);
      ppppppplVar10 = (longlong *******)ppppppplVar10[0x49];
      ppppppplVar19[1] = (longlong ******)ppppppplVar11;
      if (ppppppplVar11 == ppppppplVar10) {
        *ppppppplVar10 = (longlong ******)ppppppplVar19;
        ppppppplVar10[1] = (longlong ******)ppppppplVar19;
        ppppppplVar10[2] = (longlong ******)ppppppplVar19;
      }
      else {
        ppppppplVar9 = ppppppplVar19 + 1;
        if (bVar24) {
          ppppppplVar11[2] = (longlong ******)ppppppplVar19;
          if (ppppppplVar11 == (longlong *******)ppppppplVar10[2]) {
            ppppppplVar10[2] = (longlong ******)ppppppplVar19;
          }
        }
        else {
          *ppppppplVar11 = (longlong ******)ppppppplVar19;
          if (ppppppplVar11 == (longlong *******)*ppppppplVar10) {
            *ppppppplVar10 = (longlong ******)ppppppplVar19;
          }
        }
        ppppppplVar11 = (longlong *******)*ppppppplVar9;
        cVar3 = *(char *)(ppppppplVar11 + 3);
        while (cVar3 == '\0') {
          pppppplVar8 = ppppppplVar11[1];
          ppppppplVar20 = (longlong *******)*pppppplVar8;
          if (ppppppplVar11 == ppppppplVar20) {
            ppppppplVar20 = (longlong *******)pppppplVar8[2];
            if (*(char *)(ppppppplVar20 + 3) == '\0') goto LAB_145422e70;
            ppppppplVar9 = (longlong *******)ppppppplVar11[2];
            if (ppppppplVar19 == ppppppplVar9) {
              pppppplVar18 = *ppppppplVar9;
              ppppppplVar11[2] = pppppplVar18;
              if (*(char *)((longlong)pppppplVar18 + 0x19) == '\0') {
                pppppplVar18[1] = (longlong *****)ppppppplVar11;
                pppppplVar8 = ppppppplVar11[1];
              }
              ppppppplVar9[1] = pppppplVar8;
              if (ppppppplVar11 == (longlong *******)(*ppppppplStack_210)[1]) {
                (*ppppppplStack_210)[1] = (longlong *****)ppppppplVar9;
              }
              else if (ppppppplVar11 == (longlong *******)*pppppplVar8) {
                *pppppplVar8 = (longlong *****)ppppppplVar9;
              }
              else {
                pppppplVar8[2] = (longlong *****)ppppppplVar9;
              }
              *ppppppplVar9 = (longlong ******)ppppppplVar11;
              ppppppplVar11[1] = (longlong ******)ppppppplVar9;
              ppppppplVar19 = ppppppplVar11;
              ppppppplVar11 = ppppppplVar9;
            }
            *(undefined1 *)(ppppppplVar11 + 3) = 1;
            *(undefined1 *)(ppppppplVar19[1][1] + 3) = 0;
            ppppplVar12 = ppppppplVar19[1][1];
            ppppplVar21 = (longlong *****)*ppppplVar12;
            pppplVar4 = ppppplVar21[2];
            *ppppplVar12 = pppplVar4;
            if (*(char *)((longlong)pppplVar4 + 0x19) == '\0') {
              pppplVar4[1] = (longlong ***)ppppplVar12;
            }
            pppplVar4 = ppppplVar12[1];
            ppppplVar21[1] = pppplVar4;
            if (ppppplVar12 == (*ppppppplStack_210)[1]) {
              (*ppppppplStack_210)[1] = ppppplVar21;
              ppppplVar21[2] = (longlong ****)ppppplVar12;
            }
            else if (ppppplVar12 == (longlong *****)pppplVar4[2]) {
              pppplVar4[2] = (longlong ***)ppppplVar21;
              ppppplVar21[2] = (longlong ****)ppppplVar12;
            }
            else {
              *pppplVar4 = (longlong ***)ppppplVar21;
              ppppplVar21[2] = (longlong ****)ppppplVar12;
            }
LAB_145422fb3:
            ppppplVar12[1] = (longlong ****)ppppplVar21;
          }
          else {
            if (*(char *)(ppppppplVar20 + 3) != '\0') {
              ppppppplVar9 = (longlong *******)*ppppppplVar11;
              if (ppppppplVar19 == ppppppplVar9) {
                pppppplVar18 = ppppppplVar9[2];
                *ppppppplVar11 = pppppplVar18;
                if (*(char *)((longlong)pppppplVar18 + 0x19) == '\0') {
                  pppppplVar18[1] = (longlong *****)ppppppplVar11;
                  pppppplVar8 = ppppppplVar11[1];
                }
                ppppppplVar9[1] = pppppplVar8;
                if (ppppppplVar11 == (longlong *******)(*ppppppplStack_210)[1]) {
                  (*ppppppplStack_210)[1] = (longlong *****)ppppppplVar9;
                }
                else if (ppppppplVar11 == (longlong *******)pppppplVar8[2]) {
                  pppppplVar8[2] = (longlong *****)ppppppplVar9;
                }
                else {
                  *pppppplVar8 = (longlong *****)ppppppplVar9;
                }
                ppppppplVar9[2] = (longlong ******)ppppppplVar11;
                ppppppplVar11[1] = (longlong ******)ppppppplVar9;
                ppppppplVar19 = ppppppplVar11;
                ppppppplVar11 = ppppppplVar9;
              }
              *(undefined1 *)(ppppppplVar11 + 3) = 1;
              *(undefined1 *)(ppppppplVar19[1][1] + 3) = 0;
              ppppplVar12 = ppppppplVar19[1][1];
              ppppplVar21 = (longlong *****)ppppplVar12[2];
              pppplVar4 = *ppppplVar21;
              ppppplVar12[2] = pppplVar4;
              if (*(char *)((longlong)pppplVar4 + 0x19) == '\0') {
                pppplVar4[1] = (longlong ***)ppppplVar12;
              }
              pppplVar4 = ppppplVar12[1];
              ppppplVar21[1] = pppplVar4;
              if (ppppplVar12 == (*ppppppplStack_210)[1]) {
                (*ppppppplStack_210)[1] = ppppplVar21;
              }
              else if (ppppplVar12 == (longlong *****)*pppplVar4) {
                *pppplVar4 = (longlong ***)ppppplVar21;
              }
              else {
                pppplVar4[2] = (longlong ***)ppppplVar21;
              }
              *ppppplVar21 = (longlong ****)ppppplVar12;
              goto LAB_145422fb3;
            }
LAB_145422e70:
            *(undefined1 *)(ppppppplVar11 + 3) = 1;
            *(undefined1 *)(ppppppplVar20 + 3) = 1;
            *(undefined1 *)((*ppppppplVar9)[1] + 3) = 0;
            ppppppplVar19 = (longlong *******)(*ppppppplVar9)[1];
          }
          ppppppplVar9 = ppppppplVar19 + 1;
          ppppppplVar11 = (longlong *******)ppppppplVar19[1];
          cVar3 = *(char *)(ppppppplVar11 + 3);
        }
        ppppppplVar19 = (longlong *******)ppppppplVar10[1];
      }
      *(undefined1 *)(ppppppplVar19 + 3) = 1;
      goto LAB_145422c9a;
    }
    goto LAB_145423207;
  }
  ppppppplVar19 = (longlong *******)&ppppppplStack_c0;
  if (0xf < uStack_a8) {
    ppppppplVar19 = ppppppplStack_c0;
  }
  ppppppplVar9 = (longlong *******)ppppppplStack_58[7];
  if (ppppppplStack_58[8] < (longlong ******)0x10) {
    ppppppplVar20 = ppppppplStack_58 + 5;
  }
  else {
    ppppppplVar20 = (longlong *******)ppppppplStack_58[5];
  }
  unaff_R14 = ppppppplStack_b0;
  if (ppppppplVar9 < ppppppplStack_b0) {
    unaff_R14 = ppppppplVar9;
  }
  iVar7 = FUN_14eec62b0(ppppppplVar19,ppppppplVar20,unaff_R14);
  bVar6 = ppppppplVar10 < ppppppplVar9;
  if (iVar7 != 0) {
    bVar6 = iVar7 < 0;
  }
  if (bVar6) goto LAB_145422ab0;
  iVar7 = FUN_14eec62b0(ppppppplVar20,ppppppplVar19,unaff_R14);
  bVar6 = ppppppplVar10 <= ppppppplVar9;
  if (iVar7 != 0) {
    bVar6 = -1 < iVar7;
  }
  if ((bVar6) && (uStack_90 < *(uint *)(ppppppplStack_58 + 0xb))) goto LAB_145422ab0;
LAB_145422c9a:
  ppppppplVar19 = ppppppplStack_70;
  unaff_R14 = ppppppplStack_88;
  plVar14 = plStack_150;
  ppppppplVar10 = ppppppplStack_188;
  ppppppplStack_148 = (longlong *******)0x0;
  ppppppplStack_140 = (longlong *******)0x0;
  ppppppplStack_58[0xc] = (longlong ******)ppppppplStack_160;
  pppppplVar8 = ppppppplStack_58[0xd];
  ppppppplStack_58[0xd] = (longlong ******)ppppppplStack_80;
  ppppppplVar11 = ppppppplStack_80;
  if (pppppplVar8 != (longlong ******)0x0) {
    LOCK();
    pppppplVar18 = pppppplVar8 + 1;
    *(int *)pppppplVar18 = *(int *)pppppplVar18 + -1;
    UNLOCK();
    if (*(int *)pppppplVar18 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(pppppplVar8);
      LOCK();
      piVar1 = (int *)((longlong)pppppplVar8 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(pppppplVar8);
      }
    }
    ppppppplVar11 = (longlong *******)ppppppplStack_58[0xd];
  }
  ppppppplVar9 = ppppppplStack_140;
  if (ppppppplVar11 == (longlong *******)0x0) {
    pppppplVar8 = (longlong ******)0x0;
  }
  else {
    LOCK();
    *(int *)(ppppppplVar11 + 1) = *(int *)(ppppppplVar11 + 1) + 1;
    UNLOCK();
    pppppplVar8 = ppppppplStack_58[0xd];
  }
  *plVar14 = (longlong)ppppppplStack_58[0xc];
  plVar14[1] = (longlong)pppppplVar8;
  if (ppppppplStack_140 != (longlong *******)0x0) {
    LOCK();
    ppppppplVar11 = ppppppplStack_140 + 1;
    *(int *)ppppppplVar11 = *(int *)ppppppplVar11 + -1;
    UNLOCK();
    if (*(int *)ppppppplVar11 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppppplStack_140);
      LOCK();
      piVar1 = (int *)((longlong)ppppppplVar9 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppppplVar9);
      }
    }
  }
  ppppppplVar11 = ppppppplStack_170;
  if (ppppppplStack_170 != (longlong *******)0x0) {
    LOCK();
    ppppppplVar9 = ppppppplStack_170 + 1;
    *(int *)ppppppplVar9 = *(int *)ppppppplVar9 + -1;
    UNLOCK();
    if (*(int *)ppppppplVar9 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppppplStack_170);
      LOCK();
      piVar1 = (int *)((longlong)ppppppplVar11 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppppplVar11);
      }
    }
  }
  if (unaff_R14 != (longlong *******)0x0) {
    LOCK();
    piVar1 = (int *)((longlong)unaff_R14 + 0xc);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(unaff_R14);
    }
  }
  LOCK();
  *(int *)ppppppplVar10 = *(int *)ppppppplVar10 + -1;
  UNLOCK();
  if (*(int *)ppppppplVar10 == 0) {
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppppplVar19);
    LOCK();
    *piStack_180 = *piStack_180 + -1;
    UNLOCK();
    if (*piStack_180 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(ppppppplVar19);
    }
  }
LAB_145422e01:
  if (uStack_a8 < 0x10) {
    return plVar14;
  }
  uVar13 = uStack_a8 + 1;
  ppppppplVar11 = ppppppplStack_c0;
  if (uVar13 < 0x1000) {
LAB_145422e41:
    FUN_14e934b00(ppppppplVar11,uVar13);
    return plVar14;
  }
  ppppppplVar11 = (longlong *******)ppppppplStack_c0[-1];
  if ((ulonglong)((longlong)ppppppplStack_c0 + (-8 - (longlong)ppppppplVar11)) < 0x20) {
    uVar13 = uStack_a8 + 0x28;
    goto LAB_145422e41;
  }
LAB_14542320d:
  _invoke_watson(0,0,0,0,0);
LAB_145423226:
  FUN_1409bc820(unaff_R14);
  pcVar5 = (code *)swi(3);
  plVar14 = (longlong *)(*pcVar5)();
  return plVar14;
}

