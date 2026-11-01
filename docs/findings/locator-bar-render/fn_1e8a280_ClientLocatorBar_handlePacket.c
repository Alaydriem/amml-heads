
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141e8a280(longlong param_1,longlong param_2)

{
  char cVar1;
  uint uVar2;
  uint *puVar3;
  ulonglong uVar4;
  longlong lVar5;
  ulonglong uVar6;
  undefined8 *puVar7;
  longlong lVar8;
  longlong lVar9;
  uint *puVar10;
  uint uStack_78;
  uint uStack_74;
  uint uStack_70;
  uint uStack_6c;
  undefined8 *apuStack_60 [4];
  
  puVar10 = *(uint **)(param_2 + 0x30);
  puVar3 = *(uint **)(param_2 + 0x38);
  if (puVar10 != puVar3) {
    do {
      uStack_78 = (uint)*(ulonglong *)(puVar10 + 0x20);
      uStack_74 = puVar10[0x21];
      uVar6 = *(ulonglong *)(puVar10 + 0x20);
      uStack_70 = puVar10[0x22];
      uStack_6c = puVar10[0x23];
      lVar5 = *(longlong *)(puVar10 + 0x22);
      cVar1 = (char)puVar10[0x24];
      if (cVar1 != '\0') {
        if (cVar1 == '\x01') {
          if (*puVar10 == 0x3f) {
            lVar8 = ((lVar5 * 0x1f1f1f1f ^ uVar6) & *(ulonglong *)(param_1 + 0x38)) * 0x10;
            lVar9 = *(longlong *)(*(longlong *)(param_1 + 0x20) + 8 + lVar8);
            if (lVar9 != *(longlong *)(param_1 + 0x10)) {
              uVar4 = *(ulonglong *)(lVar9 + 0x10);
              while ((uVar6 != uVar4 || (lVar5 != *(longlong *)(lVar9 + 0x18)))) {
                if (lVar9 == *(longlong *)(*(longlong *)(param_1 + 0x20) + lVar8))
                goto LAB_141e8a377;
                lVar9 = *(longlong *)(lVar9 + 8);
                uVar4 = *(ulonglong *)(lVar9 + 0x10);
              }
              if ((lVar9 != *(longlong *)(param_1 + 0x10)) && (*(longlong *)(lVar9 + 0x20) != 0))
              goto LAB_141e8a2cb;
            }
LAB_141e8a377:
            if ((char)puVar10[0x1e] == '\x01') {
              puVar7 = (undefined8 *)
                       (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,0x98);
              if (puVar7 == (undefined8 *)0x0) {
                FUN_1409bc820(0x98);
              }
              puVar7[1] = 0;
              puVar7[2] = 0xffffffff00000000;
              *(undefined1 *)(puVar7 + 5) = 0;
              *(undefined1 *)(puVar7 + 10) = 0;
              puVar7[0xb] = 0;
              *(undefined2 *)(puVar7 + 0xc) = 0;
              *(undefined1 *)((longlong)puVar7 + 0x74) = 0;
              *puVar7 = &UNK_14f13dfd0;
              puVar7[0xf] = *(undefined8 *)(puVar10 + 0x1c);
              puVar7[0x10] = 0;
              puVar7[0x11] = 0;
              *(undefined4 *)(puVar7 + 0x12) = 0xffffffff;
            }
            else {
              puVar7 = (undefined8 *)
                       (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,0x78);
              if (puVar7 == (undefined8 *)0x0) {
                FUN_1409bc820(0x78);
              }
              puVar7[2] = 0;
              puVar7[3] = 0;
              puVar7[10] = 0;
              puVar7[0xb] = 0;
              puVar7[0xc] = 0;
              puVar7[0xd] = 0;
              *puVar7 = 0;
              puVar7[1] = 0;
              puVar7[8] = 0;
              puVar7[9] = 0;
              puVar7[6] = 0;
              puVar7[7] = 0;
              puVar7[4] = 0;
              puVar7[5] = 0;
              puVar7[0xe] = 0;
              *(undefined4 *)((longlong)puVar7 + 0x14) = 0xffffffff;
              puVar7[0xb] = 0;
              *(undefined2 *)(puVar7 + 0xc) = 0;
              *puVar7 = &UNK_14f114660;
            }
            apuStack_60[0] = puVar7;
            FUN_142514a90(param_1,&uStack_78,apuStack_60);
            goto LAB_141e8a46e;
          }
        }
        else if (cVar1 == '\x02') {
          FUN_140c281b0(param_1 + 8,&uStack_78);
        }
        else {
LAB_141e8a46e:
          lVar9 = ((CONCAT44(uStack_6c,uStack_70) * 0x1f1f1f1f ^ CONCAT44(uStack_74,uStack_78)) &
                  *(ulonglong *)(param_1 + 0x38)) * 0x10;
          lVar5 = *(longlong *)(*(longlong *)(param_1 + 0x20) + 8 + lVar9);
          if (lVar5 != *(longlong *)(param_1 + 0x10)) {
            uVar6 = *(ulonglong *)(lVar5 + 0x10);
            while ((CONCAT44(uStack_74,uStack_78) != uVar6 ||
                   (CONCAT44(uStack_6c,uStack_70) != *(longlong *)(lVar5 + 0x18)))) {
              if (lVar5 == *(longlong *)(*(longlong *)(param_1 + 0x20) + lVar9)) goto LAB_141e8a2cb;
              lVar5 = *(longlong *)(lVar5 + 8);
              uVar6 = *(ulonglong *)(lVar5 + 0x10);
            }
            if ((lVar5 != *(longlong *)(param_1 + 0x10)) &&
               (lVar5 = *(longlong *)(lVar5 + 0x20), lVar5 != 0)) {
              uVar2 = *puVar10;
              if ((uVar2 & 1) != 0) {
                if ((char)puVar10[6] != '\x01') goto LAB_141e8a2cb;
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar5,puVar10 + 2);
              }
              if ((uVar2 & 0x10) != 0) {
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar5,puVar10 + 0x15);
              }
              if ((uVar2 & 2) != 0) {
                if (*(char *)((longlong)puVar10 + 5) != '\x01') goto LAB_141e8a2cb;
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar5,(char)puVar10[1]);
              }
              if ((uVar2 & 4) != 0) {
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar5,puVar10 + 8);
              }
              if ((uVar2 & 8) != 0) {
                if ((char)puVar10[0x14] != '\x01') goto LAB_141e8a2cb;
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar5,puVar10 + 0x12);
              }
              if (((uVar2 & 0x20) != 0) && (*(char *)((longlong)puVar10 + 0x69) == '\x01')) {
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar5,(char)puVar10[0x1a]);
              }
            }
          }
        }
      }
LAB_141e8a2cb:
      puVar10 = puVar10 + 0x26;
    } while (puVar10 != puVar3);
  }
  return;
}

