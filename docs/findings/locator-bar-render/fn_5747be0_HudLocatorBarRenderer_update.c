
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Restarted to delay deadcode elimination for space: ram */

undefined8 FUN_145747be0(longlong param_1,undefined8 param_2,longlong param_3)

{
  int *piVar1;
  code *pcVar2;
  float *pfVar3;
  longlong lVar4;
  char *pcVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  ulonglong uVar9;
  undefined8 uVar10;
  longlong lVar11;
  float fVar12;
  float fVar13;
  undefined8 in_stack_fffffffffffffa78;
  undefined4 uVar14;
  float in_stack_fffffffffffffa80;
  uint uVar15;
  undefined8 uStack_568;
  undefined4 uStack_560;
  undefined1 uStack_55c;
  undefined1 uStack_55b;
  undefined2 uStack_55a;
  undefined1 uStack_558;
  undefined1 uStack_557;
  undefined2 uStack_556;
  undefined1 uStack_554;
  undefined1 uStack_553;
  undefined2 uStack_552;
  undefined1 uStack_550;
  undefined1 uStack_54f;
  undefined2 uStack_54e;
  undefined1 uStack_54c;
  undefined1 uStack_54b;
  undefined1 auStack_54a [994];
  undefined8 uStack_168;
  undefined1 auStack_160 [16];
  char *pcStack_150;
  undefined8 uStack_148;
  char *pcStack_140;
  undefined8 uStack_138;
  char *pcStack_130;
  undefined8 uStack_128;
  char *pcStack_120;
  undefined8 uStack_118;
  undefined8 *puStack_110;
  undefined8 uStack_108;
  char *pcStack_100;
  undefined8 uStack_f8;
  undefined1 auStack_f0 [8];
  longlong lStack_e8;
  undefined1 uStack_d8;
  undefined7 uStack_d7;
  ulonglong uStack_d0;
  undefined8 uStack_c8;
  ulonglong uStack_c0;
  longlong lStack_a0;
  longlong lStack_98;
  char *pcStack_90;
  longlong lStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  ulonglong uStack_58;
  undefined8 uStack_40;
  
  uVar14 = (undefined4)((ulonglong)in_stack_fffffffffffffa78 >> 0x20);
  uStack_40 = 0xfffffffffffffffe;
  uVar10 = *(undefined8 *)(param_3 + 0x48);
  fVar12 = (float)uVar10;
  fVar13 = (float)((ulonglong)uVar10 >> 0x20);
  if (((*(longlong *)(param_1 + 0x10) == 0) ||
      (_DAT_14efd23d0 < (float)((uint)(*(float *)(param_1 + 0xf20) - fVar12) & _DAT_14eee6c10))) ||
     (_DAT_14efd23d0 < (float)((uint)(*(float *)(param_1 + 0xf24) - fVar13) & _DAT_14eee6c10))) {
    FUN_14eec62d0(&uStack_552,0,0x3ea);
    uStack_558 = 0x74;
    uStack_557 = 0x6f;
    uStack_556 = 0x5f72;
    uStack_554 = 0x62;
    uStack_553 = 0x67;
    uStack_568 = CONCAT44(_UNK_14f5b8957,_DAT_14f5b8953);
    uStack_560 = _UNK_14f5b895b;
    uStack_55c = (undefined1)_UNK_14f5b895f;
    uStack_55b = (undefined1)((uint)_UNK_14f5b895f >> 8);
    uStack_55a = (undefined2)((uint)_UNK_14f5b895f >> 0x10);
    uStack_168 = 0x16;
    puStack_110 = &uStack_568;
    uStack_108 = 0x16;
    FUN_1403946b0(&uStack_78,&puStack_110);
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_2,auStack_160);
    FUN_141913f40(auStack_f0,auStack_160,&uStack_78);
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_2,&pcStack_90);
    if (pcStack_90 == (char *)0x0) {
      _DAT_00000000 = 0xdeadc0de;
    }
    else if (*pcStack_90 == '\0') {
      _DAT_00000000 = 0xdeadc0de;
    }
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_80,&uStack_568,&uStack_78,0);
    lVar11 = lStack_88;
    pcStack_90 = (char *)0x0;
    lStack_88 = 0;
    if (lVar11 != 0) {
      LOCK();
      piVar1 = (int *)(lVar11 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
      lVar11 = lStack_88;
      uStack_80 = 0;
      if (lStack_88 != 0) {
        LOCK();
        piVar1 = (int *)(lStack_88 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_88);
          LOCK();
          piVar1 = (int *)(lVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          }
        }
      }
    }
    pfVar3 = (float *)CONCAT26(uStack_55a,CONCAT15(uStack_55b,CONCAT14(uStack_55c,uStack_560)));
    if (pfVar3 != (float *)0x0) {
      pcStack_90 = (char *)0x0;
      lStack_88 = CONCAT44((int)(float)((ulonglong)*(undefined8 *)(pfVar3 + 5) >> 0x20),
                           (int)(float)*(undefined8 *)(pfVar3 + 5));
      FUN_141914350(auStack_f0,&pcStack_100,&pcStack_90,(int)*pfVar3,CONCAT44(uVar14,(int)pfVar3[1])
                    ,fVar12,fVar13);
      pcVar5 = pcStack_100;
      pcStack_100 = (char *)0x0;
      lVar11 = *(longlong *)(param_1 + 0x10);
      *(char **)(param_1 + 0x10) = pcVar5;
      if ((lVar11 != 0) &&
         ((*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11,1), pcStack_100 != (char *)0x0)) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(pcStack_100,1);
      }
      *(undefined8 *)(param_1 + 0xf20) = uVar10;
      in_stack_fffffffffffffa80 = fVar12;
    }
    FUN_1401103c0(&uStack_568);
    if (0xf < uStack_c0) {
      lVar4 = CONCAT71(uStack_d7,uStack_d8);
      uVar9 = uStack_c0 + 1;
      lVar11 = lVar4;
      if (0xfff < uVar9) {
        lVar11 = *(longlong *)(lVar4 + -8);
        if (0x1f < (ulonglong)((lVar4 + -8) - lVar11)) goto LAB_145748b48;
        uVar9 = uStack_c0 + 0x28;
      }
      FUN_14e934b00(lVar11,uVar9);
    }
    lVar11 = lStack_e8;
    uStack_c8 = 0;
    uStack_c0 = 0xf;
    uStack_d8 = 0;
    if (lStack_e8 != 0) {
      LOCK();
      piVar1 = (int *)(lStack_e8 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_e8);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
    }
    if (0xf < uStack_58) {
      uVar9 = uStack_58 + 1;
      lVar11 = uStack_70;
      if (0xfff < uVar9) {
        lVar11 = *(longlong *)(uStack_70 + -8);
        if (0x1f < (ulonglong)((uStack_70 + -8) - lVar11)) goto LAB_145748b48;
        uVar9 = uStack_58 + 0x28;
      }
      FUN_14e934b00(lVar11,uVar9);
    }
  }
  if (*(char *)(param_1 + 0x18) == '\0') {
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_2,&lStack_a0);
    lVar11 = lStack_a0;
    FUN_14eec62d0(&uStack_54c,0,0x3e4);
    uStack_568 = _DAT_14f572d66;
    uStack_560 = _UNK_14f572d6e;
    uStack_55c = (undefined1)_DAT_14f572d72;
    uStack_55b = (undefined1)((uint)_DAT_14f572d72 >> 8);
    uStack_55a = (undefined2)((uint)_DAT_14f572d72 >> 0x10);
    uStack_558 = (undefined1)_UNK_14f572d76;
    uStack_557 = (undefined1)((uint)_UNK_14f572d76 >> 8);
    uStack_556 = (undefined2)((uint)_UNK_14f572d76 >> 0x10);
    uStack_554 = (undefined1)_UNK_14f572d7a;
    uStack_553 = (undefined1)((uint)_UNK_14f572d7a >> 8);
    uStack_552 = (undefined2)((uint)_UNK_14f572d7a >> 0x10);
    uStack_550 = (undefined1)_UNK_14f572d7e;
    uStack_54f = (undefined1)((uint)_UNK_14f572d7e >> 8);
    uStack_54e = (undefined2)((uint)_UNK_14f572d7e >> 0x10);
    uStack_168 = 0x1c;
    lStack_88 = 0x1c;
    pcStack_90 = (char *)&uStack_568;
    FUN_1403946b0(auStack_f0,&pcStack_90);
    uVar15 = (uint)in_stack_fffffffffffffa80 & 0xffffff00;
    FUN_14541f580(lVar11 + 0x18,&uStack_78,auStack_f0,0,0,uVar15);
    if ((undefined8 *)(param_1 + 0x20) != &uStack_78) {
      uVar14 = (undefined4)uStack_78;
      uVar6 = uStack_78._4_4_;
      uVar7 = (undefined4)uStack_70;
      uVar8 = uStack_70._4_4_;
      uStack_78 = 0;
      uStack_70 = 0;
      lVar11 = *(longlong *)(param_1 + 0x28);
      *(undefined4 *)(param_1 + 0x20) = uVar14;
      *(undefined4 *)(param_1 + 0x24) = uVar6;
      *(undefined4 *)(param_1 + 0x28) = uVar7;
      *(undefined4 *)(param_1 + 0x2c) = uVar8;
      if (lVar11 != 0) {
        LOCK();
        piVar1 = (int *)(lVar11 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          LOCK();
          piVar1 = (int *)(lVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          }
        }
      }
      uVar14 = (undefined4)uStack_68;
      uVar6 = uStack_68._4_4_;
      uVar7 = (undefined4)uStack_60;
      uVar8 = uStack_60._4_4_;
      uStack_68 = 0;
      uStack_60 = 0;
      lVar11 = *(longlong *)(param_1 + 0x38);
      *(undefined4 *)(param_1 + 0x30) = uVar14;
      *(undefined4 *)(param_1 + 0x34) = uVar6;
      *(undefined4 *)(param_1 + 0x38) = uVar7;
      *(undefined4 *)(param_1 + 0x3c) = uVar8;
      if (lVar11 != 0) {
        LOCK();
        piVar1 = (int *)(lVar11 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          LOCK();
          piVar1 = (int *)(lVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          }
        }
      }
    }
    lVar11 = uStack_60;
    if (uStack_60 != 0) {
      LOCK();
      piVar1 = (int *)(uStack_60 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_60);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
    }
    lVar11 = uStack_70;
    if (uStack_70 != 0) {
      LOCK();
      piVar1 = (int *)(uStack_70 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_70);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
    }
    if (0xf < uStack_d0) {
      uVar9 = uStack_d0 + 1;
      lVar11 = lStack_e8;
      if (0xfff < uVar9) {
        lVar11 = *(longlong *)(lStack_e8 + -8);
        if (0x1f < (ulonglong)((lStack_e8 + -8) - lVar11)) goto LAB_145748b48;
        uVar9 = uStack_d0 + 0x28;
      }
      FUN_14e934b00(lVar11,uVar9);
    }
    lVar11 = lStack_a0;
    FUN_14eec62d0(auStack_54a,0,0x3e2);
    uStack_568 = _DAT_14f57fb44;
    uStack_560 = (undefined4)_UNK_14f57fb4c;
    uStack_55c = (undefined1)((uint6)_UNK_14f57fb4c >> 0x20);
    uStack_55b = (undefined1)((uint6)_UNK_14f57fb4c >> 0x28);
    uStack_55a = _DAT_14f57fb52;
    uStack_558 = (undefined1)_UNK_14f57fb54;
    uStack_557 = (undefined1)((ushort)_UNK_14f57fb54 >> 8);
    uStack_556 = (undefined2)_UNK_14f57fb56;
    uStack_554 = (undefined1)((uint)_UNK_14f57fb56 >> 0x10);
    uStack_553 = (undefined1)((uint)_UNK_14f57fb56 >> 0x18);
    uStack_552 = (undefined2)_UNK_14f57fb5a;
    uStack_550 = (undefined1)((uint)_UNK_14f57fb5a >> 0x10);
    uStack_54f = (undefined1)((uint)_UNK_14f57fb5a >> 0x18);
    uStack_54e = (undefined2)_UNK_14f57fb5e;
    uStack_54c = (undefined1)((uint)_UNK_14f57fb5e >> 0x10);
    uStack_54b = (undefined1)((uint)_UNK_14f57fb5e >> 0x18);
    uStack_168 = 0x1e;
    uStack_f8 = 0x1e;
    pcStack_100 = (char *)&uStack_568;
    FUN_1403946b0(auStack_f0,&pcStack_100);
    uVar15 = uVar15 & 0xffffff00;
    FUN_14541f580(lVar11 + 0x18,&uStack_78,auStack_f0,0,0,uVar15);
    if ((undefined8 *)(param_1 + 0x40) != &uStack_78) {
      uVar14 = (undefined4)uStack_78;
      uVar6 = uStack_78._4_4_;
      uVar7 = (undefined4)uStack_70;
      uVar8 = uStack_70._4_4_;
      uStack_78 = 0;
      uStack_70 = 0;
      lVar11 = *(longlong *)(param_1 + 0x48);
      *(undefined4 *)(param_1 + 0x40) = uVar14;
      *(undefined4 *)(param_1 + 0x44) = uVar6;
      *(undefined4 *)(param_1 + 0x48) = uVar7;
      *(undefined4 *)(param_1 + 0x4c) = uVar8;
      if (lVar11 != 0) {
        LOCK();
        piVar1 = (int *)(lVar11 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          LOCK();
          piVar1 = (int *)(lVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          }
        }
      }
      uVar14 = (undefined4)uStack_68;
      uVar6 = uStack_68._4_4_;
      uVar7 = (undefined4)uStack_60;
      uVar8 = uStack_60._4_4_;
      uStack_68 = 0;
      uStack_60 = 0;
      lVar11 = *(longlong *)(param_1 + 0x58);
      *(undefined4 *)(param_1 + 0x50) = uVar14;
      *(undefined4 *)(param_1 + 0x54) = uVar6;
      *(undefined4 *)(param_1 + 0x58) = uVar7;
      *(undefined4 *)(param_1 + 0x5c) = uVar8;
      if (lVar11 != 0) {
        LOCK();
        piVar1 = (int *)(lVar11 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          LOCK();
          piVar1 = (int *)(lVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          }
        }
      }
    }
    lVar11 = uStack_60;
    if (uStack_60 != 0) {
      LOCK();
      piVar1 = (int *)(uStack_60 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_60);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
    }
    lVar11 = uStack_70;
    if (uStack_70 != 0) {
      LOCK();
      piVar1 = (int *)(uStack_70 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_70);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
    }
    if (0xf < uStack_d0) {
      uVar9 = uStack_d0 + 1;
      lVar11 = lStack_e8;
      if (0xfff < uVar9) {
        lVar11 = *(longlong *)(lStack_e8 + -8);
        if (0x1f < (ulonglong)((lStack_e8 + -8) - lVar11)) goto LAB_145748b48;
        uVar9 = uStack_d0 + 0x28;
      }
      FUN_14e934b00(lVar11,uVar9);
    }
    lVar11 = lStack_a0;
    FUN_14eec62d0(auStack_54a,0,0x3e2);
    uStack_568 = _DAT_14f7e3479;
    uStack_560 = (undefined4)_UNK_14f7e3481;
    uStack_55c = (undefined1)((uint5)_UNK_14f7e3481 >> 0x20);
    uStack_55b = (undefined1)_DAT_14f7e3486;
    uStack_55a = (undefined2)((uint)_DAT_14f7e3486 >> 8);
    uStack_558 = (undefined1)((uint)_DAT_14f7e3486 >> 0x18);
    uStack_557 = (undefined1)_UNK_14f7e348a;
    uStack_556 = (undefined2)((uint)_UNK_14f7e348a >> 8);
    uStack_554 = (undefined1)((uint)_UNK_14f7e348a >> 0x18);
    uStack_553 = (undefined1)_UNK_14f7e348e;
    uStack_552 = (undefined2)((uint)_UNK_14f7e348e >> 8);
    uStack_550 = (undefined1)((uint)_UNK_14f7e348e >> 0x18);
    uStack_54f = (undefined1)_UNK_14f7e3492;
    uStack_54e = (undefined2)((uint)_UNK_14f7e3492 >> 8);
    uStack_54c = (undefined1)((uint)_UNK_14f7e3492 >> 0x18);
    uStack_168 = 0x1d;
    uStack_54b = 0;
    uStack_148 = 0x1d;
    pcStack_150 = (char *)&uStack_568;
    FUN_1403946b0(auStack_f0,&pcStack_150);
    uVar15 = uVar15 & 0xffffff00;
    FUN_14541f580(lVar11 + 0x18,&uStack_78,auStack_f0,0,0,uVar15);
    if ((undefined8 *)(param_1 + 0x60) != &uStack_78) {
      uVar14 = (undefined4)uStack_78;
      uVar6 = uStack_78._4_4_;
      uVar7 = (undefined4)uStack_70;
      uVar8 = uStack_70._4_4_;
      uStack_78 = 0;
      uStack_70 = 0;
      lVar11 = *(longlong *)(param_1 + 0x68);
      *(undefined4 *)(param_1 + 0x60) = uVar14;
      *(undefined4 *)(param_1 + 100) = uVar6;
      *(undefined4 *)(param_1 + 0x68) = uVar7;
      *(undefined4 *)(param_1 + 0x6c) = uVar8;
      if (lVar11 != 0) {
        LOCK();
        piVar1 = (int *)(lVar11 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          LOCK();
          piVar1 = (int *)(lVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          }
        }
      }
      uVar14 = (undefined4)uStack_68;
      uVar6 = uStack_68._4_4_;
      uVar7 = (undefined4)uStack_60;
      uVar8 = uStack_60._4_4_;
      uStack_68 = 0;
      uStack_60 = 0;
      lVar11 = *(longlong *)(param_1 + 0x78);
      *(undefined4 *)(param_1 + 0x70) = uVar14;
      *(undefined4 *)(param_1 + 0x74) = uVar6;
      *(undefined4 *)(param_1 + 0x78) = uVar7;
      *(undefined4 *)(param_1 + 0x7c) = uVar8;
      if (lVar11 != 0) {
        LOCK();
        piVar1 = (int *)(lVar11 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          LOCK();
          piVar1 = (int *)(lVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          }
        }
      }
    }
    lVar11 = uStack_60;
    if (uStack_60 != 0) {
      LOCK();
      piVar1 = (int *)(uStack_60 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_60);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
    }
    lVar11 = uStack_70;
    if (uStack_70 != 0) {
      LOCK();
      piVar1 = (int *)(uStack_70 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_70);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
    }
    if (0xf < uStack_d0) {
      uVar9 = uStack_d0 + 1;
      lVar11 = lStack_e8;
      if (0xfff < uVar9) {
        lVar11 = *(longlong *)(lStack_e8 + -8);
        if (0x1f < (ulonglong)((lStack_e8 + -8) - lVar11)) goto LAB_145748b48;
        uVar9 = uStack_d0 + 0x28;
      }
      FUN_14e934b00(lVar11,uVar9);
    }
    lVar11 = lStack_a0;
    FUN_14eec62d0(auStack_54a,0,0x3e2);
    uStack_568 = _DAT_14f7de056;
    uStack_560 = (undefined4)_UNK_14f7de05e;
    uStack_55c = (undefined1)((uint5)_UNK_14f7de05e >> 0x20);
    uStack_55b = (undefined1)_DAT_14f7de063;
    uStack_55a = (undefined2)((uint)_DAT_14f7de063 >> 8);
    uStack_558 = (undefined1)((uint)_DAT_14f7de063 >> 0x18);
    uStack_557 = (undefined1)_UNK_14f7de067;
    uStack_556 = (undefined2)((uint)_UNK_14f7de067 >> 8);
    uStack_554 = (undefined1)((uint)_UNK_14f7de067 >> 0x18);
    uStack_553 = (undefined1)_UNK_14f7de06b;
    uStack_552 = (undefined2)((uint)_UNK_14f7de06b >> 8);
    uStack_550 = (undefined1)((uint)_UNK_14f7de06b >> 0x18);
    uStack_54f = (undefined1)_UNK_14f7de06f;
    uStack_54e = (undefined2)((uint)_UNK_14f7de06f >> 8);
    uStack_54c = (undefined1)((uint)_UNK_14f7de06f >> 0x18);
    uStack_168 = 0x1d;
    uStack_54b = 0;
    uStack_138 = 0x1d;
    pcStack_140 = (char *)&uStack_568;
    FUN_1403946b0(auStack_f0,&pcStack_140);
    uVar15 = uVar15 & 0xffffff00;
    FUN_14541f580(lVar11 + 0x18,&uStack_78,auStack_f0,0,0,uVar15);
    if ((undefined8 *)(param_1 + 0x80) != &uStack_78) {
      uVar14 = (undefined4)uStack_78;
      uVar6 = uStack_78._4_4_;
      uVar7 = (undefined4)uStack_70;
      uVar8 = uStack_70._4_4_;
      uStack_78 = 0;
      uStack_70 = 0;
      lVar11 = *(longlong *)(param_1 + 0x88);
      *(undefined4 *)(param_1 + 0x80) = uVar14;
      *(undefined4 *)(param_1 + 0x84) = uVar6;
      *(undefined4 *)(param_1 + 0x88) = uVar7;
      *(undefined4 *)(param_1 + 0x8c) = uVar8;
      if (lVar11 != 0) {
        LOCK();
        piVar1 = (int *)(lVar11 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          LOCK();
          piVar1 = (int *)(lVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          }
        }
      }
      uVar14 = (undefined4)uStack_68;
      uVar6 = uStack_68._4_4_;
      uVar7 = (undefined4)uStack_60;
      uVar8 = uStack_60._4_4_;
      uStack_68 = 0;
      uStack_60 = 0;
      lVar11 = *(longlong *)(param_1 + 0x98);
      *(undefined4 *)(param_1 + 0x90) = uVar14;
      *(undefined4 *)(param_1 + 0x94) = uVar6;
      *(undefined4 *)(param_1 + 0x98) = uVar7;
      *(undefined4 *)(param_1 + 0x9c) = uVar8;
      if (lVar11 != 0) {
        LOCK();
        piVar1 = (int *)(lVar11 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          LOCK();
          piVar1 = (int *)(lVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          }
        }
      }
    }
    lVar11 = uStack_60;
    if (uStack_60 != 0) {
      LOCK();
      piVar1 = (int *)(uStack_60 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_60);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
    }
    lVar11 = uStack_70;
    if (uStack_70 != 0) {
      LOCK();
      piVar1 = (int *)(uStack_70 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_70);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
    }
    if (0xf < uStack_d0) {
      uVar9 = uStack_d0 + 1;
      lVar11 = lStack_e8;
      if (0xfff < uVar9) {
        lVar11 = *(longlong *)(lStack_e8 + -8);
        if (0x1f < (ulonglong)((lStack_e8 + -8) - lVar11)) goto LAB_145748b48;
        uVar9 = uStack_d0 + 0x28;
      }
      FUN_14e934b00(lVar11,uVar9);
    }
    lVar11 = lStack_a0;
    FUN_14eec62d0(auStack_54a,0,0x3e2);
    uStack_568 = _DAT_14f7d995d;
    uStack_560 = (undefined4)_UNK_14f7d9965;
    uStack_55c = (undefined1)((uint5)_UNK_14f7d9965 >> 0x20);
    uStack_55b = (undefined1)_DAT_14f7d996a;
    uStack_55a = (undefined2)((uint)_DAT_14f7d996a >> 8);
    uStack_558 = (undefined1)((uint)_DAT_14f7d996a >> 0x18);
    uStack_557 = (undefined1)_UNK_14f7d996e;
    uStack_556 = (undefined2)((uint)_UNK_14f7d996e >> 8);
    uStack_554 = (undefined1)((uint)_UNK_14f7d996e >> 0x18);
    uStack_553 = (undefined1)_UNK_14f7d9972;
    uStack_552 = (undefined2)((uint)_UNK_14f7d9972 >> 8);
    uStack_550 = (undefined1)((uint)_UNK_14f7d9972 >> 0x18);
    uStack_54f = (undefined1)_UNK_14f7d9976;
    uStack_54e = (undefined2)((uint)_UNK_14f7d9976 >> 8);
    uStack_54c = (undefined1)((uint)_UNK_14f7d9976 >> 0x18);
    uStack_168 = 0x1d;
    uStack_54b = 0;
    uStack_128 = 0x1d;
    pcStack_130 = (char *)&uStack_568;
    FUN_1403946b0(auStack_f0,&pcStack_130);
    uVar15 = uVar15 & 0xffffff00;
    FUN_14541f580(lVar11 + 0x18,&uStack_78,auStack_f0,0,0,uVar15);
    if ((undefined8 *)(param_1 + 0xa0) != &uStack_78) {
      uVar14 = (undefined4)uStack_78;
      uVar6 = uStack_78._4_4_;
      uVar7 = (undefined4)uStack_70;
      uVar8 = uStack_70._4_4_;
      uStack_78 = 0;
      uStack_70 = 0;
      lVar11 = *(longlong *)(param_1 + 0xa8);
      *(undefined4 *)(param_1 + 0xa0) = uVar14;
      *(undefined4 *)(param_1 + 0xa4) = uVar6;
      *(undefined4 *)(param_1 + 0xa8) = uVar7;
      *(undefined4 *)(param_1 + 0xac) = uVar8;
      if (lVar11 != 0) {
        LOCK();
        piVar1 = (int *)(lVar11 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          LOCK();
          piVar1 = (int *)(lVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          }
        }
      }
      uVar14 = (undefined4)uStack_68;
      uVar6 = uStack_68._4_4_;
      uVar7 = (undefined4)uStack_60;
      uVar8 = uStack_60._4_4_;
      uStack_68 = 0;
      uStack_60 = 0;
      lVar11 = *(longlong *)(param_1 + 0xb8);
      *(undefined4 *)(param_1 + 0xb0) = uVar14;
      *(undefined4 *)(param_1 + 0xb4) = uVar6;
      *(undefined4 *)(param_1 + 0xb8) = uVar7;
      *(undefined4 *)(param_1 + 0xbc) = uVar8;
      if (lVar11 != 0) {
        LOCK();
        piVar1 = (int *)(lVar11 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          LOCK();
          piVar1 = (int *)(lVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          }
        }
      }
    }
    lVar11 = uStack_60;
    if (uStack_60 != 0) {
      LOCK();
      piVar1 = (int *)(uStack_60 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_60);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
    }
    lVar11 = uStack_70;
    if (uStack_70 != 0) {
      LOCK();
      piVar1 = (int *)(uStack_70 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_70);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
    }
    if (0xf < uStack_d0) {
      uVar9 = uStack_d0 + 1;
      lVar11 = lStack_e8;
      if (0xfff < uVar9) {
        lVar11 = *(longlong *)(lStack_e8 + -8);
        if (0x1f < (ulonglong)((lStack_e8 + -8) - lVar11)) goto LAB_145748b48;
        uVar9 = uStack_d0 + 0x28;
      }
      FUN_14e934b00(lVar11,uVar9);
    }
    FUN_14eec62d0(auStack_54a,0,0x3e2);
    uStack_568 = _DAT_14f7d6504;
    uStack_560 = (undefined4)_UNK_14f7d650c;
    uStack_55c = (undefined1)((uint5)_UNK_14f7d650c >> 0x20);
    uStack_55b = (undefined1)_DAT_14f7d6511;
    uStack_55a = (undefined2)((uint)_DAT_14f7d6511 >> 8);
    uStack_558 = (undefined1)((uint)_DAT_14f7d6511 >> 0x18);
    uStack_557 = (undefined1)_UNK_14f7d6515;
    uStack_556 = (undefined2)((uint)_UNK_14f7d6515 >> 8);
    uStack_554 = (undefined1)((uint)_UNK_14f7d6515 >> 0x18);
    uStack_553 = (undefined1)_UNK_14f7d6519;
    uStack_552 = (undefined2)((uint)_UNK_14f7d6519 >> 8);
    uStack_550 = (undefined1)((uint)_UNK_14f7d6519 >> 0x18);
    uStack_54f = (undefined1)_UNK_14f7d651d;
    uStack_54e = (undefined2)((uint)_UNK_14f7d651d >> 8);
    uStack_54c = (undefined1)((uint)_UNK_14f7d651d >> 0x18);
    uStack_168 = 0x1d;
    uStack_54b = 0;
    uStack_118 = 0x1d;
    pcStack_120 = (char *)&uStack_568;
    FUN_1403946b0(auStack_f0,&pcStack_120);
    FUN_14541f580(lStack_a0 + 0x18,&uStack_78,auStack_f0,0,0,uVar15 & 0xffffff00);
    if ((undefined8 *)(param_1 + 0xc0) != &uStack_78) {
      uVar14 = (undefined4)uStack_78;
      uVar6 = uStack_78._4_4_;
      uVar7 = (undefined4)uStack_70;
      uVar8 = uStack_70._4_4_;
      uStack_78 = 0;
      uStack_70 = 0;
      lVar11 = *(longlong *)(param_1 + 200);
      *(undefined4 *)(param_1 + 0xc0) = uVar14;
      *(undefined4 *)(param_1 + 0xc4) = uVar6;
      *(undefined4 *)(param_1 + 200) = uVar7;
      *(undefined4 *)(param_1 + 0xcc) = uVar8;
      if (lVar11 != 0) {
        LOCK();
        piVar1 = (int *)(lVar11 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          LOCK();
          piVar1 = (int *)(lVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          }
        }
      }
      uVar14 = (undefined4)uStack_68;
      uVar6 = uStack_68._4_4_;
      uVar7 = (undefined4)uStack_60;
      uVar8 = uStack_60._4_4_;
      uStack_68 = 0;
      uStack_60 = 0;
      lVar11 = *(longlong *)(param_1 + 0xd8);
      *(undefined4 *)(param_1 + 0xd0) = uVar14;
      *(undefined4 *)(param_1 + 0xd4) = uVar6;
      *(undefined4 *)(param_1 + 0xd8) = uVar7;
      *(undefined4 *)(param_1 + 0xdc) = uVar8;
      if (lVar11 != 0) {
        LOCK();
        piVar1 = (int *)(lVar11 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          LOCK();
          piVar1 = (int *)(lVar11 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
          }
        }
      }
    }
    lVar11 = uStack_60;
    if (uStack_60 != 0) {
      LOCK();
      piVar1 = (int *)(uStack_60 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_60);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
    }
    lVar11 = uStack_70;
    if (uStack_70 != 0) {
      LOCK();
      piVar1 = (int *)(uStack_70 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_70);
        LOCK();
        piVar1 = (int *)(lVar11 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar11);
        }
      }
    }
    if (0xf < uStack_d0) {
      uVar9 = uStack_d0 + 1;
      lVar11 = lStack_e8;
      if (0xfff < uVar9) {
        lVar11 = *(longlong *)(lStack_e8 + -8);
        if (0x1f < (ulonglong)((lStack_e8 + -8) - lVar11)) {
LAB_145748b48:
          _invoke_watson(0,0,0,0,0);
          pcVar2 = (code *)swi(3);
          uVar10 = (*pcVar2)();
          return uVar10;
        }
        uVar9 = uStack_d0 + 0x28;
      }
      FUN_14e934b00(lVar11,uVar9);
    }
    if (lStack_98 != 0) {
      LOCK();
      piVar1 = (int *)(lStack_98 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_98);
        LOCK();
        piVar1 = (int *)(lStack_98 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_98);
        }
      }
    }
    *(undefined1 *)(param_1 + 0x18) = 1;
  }
  return 1;
}

