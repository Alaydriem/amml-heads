
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int * FUN_145a4c000(int *param_1,int *param_2)

{
  longlong lVar1;
  longlong lVar2;
  undefined4 uVar3;
  char cVar4;
  int iVar5;
  
  *(code **)param_1 = FUN_1403c7b10;
  param_1[2] = 0;
  param_1[3] = 0;
  param_1[4] = 0;
  param_1[5] = 0;
  *(undefined8 *)((longlong)param_1 + 0x17) = 0;
  *(undefined8 *)((longlong)param_1 + 0x1f) = 0;
  param_1[10] = 1;
  uVar3 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1b0,0x53);
  if (param_1 != param_2 + 6) {
    lVar1 = *(longlong *)(param_2 + 8);
    param_2[8] = 0;
    param_2[9] = 0;
    lVar2 = *(longlong *)(param_1 + 2);
    *(longlong *)(param_1 + 2) = lVar1;
    if (lVar2 != 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    }
    *(undefined8 *)param_1 = *(undefined8 *)(param_2 + 6);
    *(undefined8 *)(param_1 + 4) = *(undefined8 *)(param_2 + 10);
    param_2[10] = 0;
    param_2[0xb] = 0;
  }
  param_1[6] = param_2[1];
  param_1[7] = param_2[2];
  if (param_2[3] != 0) {
    param_1[10] = param_2[3];
  }
  iVar5 = 0;
  if (*param_2 - 1U < 5) {
    iVar5 = *(int *)(&UNK_14f0a2e08 + (ulonglong)(*param_2 - 1U) * 4);
  }
  param_1[8] = iVar5;
  cVar4 = '\x01';
  if ((char)param_2[4] != '\x01') {
    cVar4 = ((char)param_2[4] == '\x02') * '\x02';
  }
  *(char *)(param_1 + 9) = cVar4;
  param_2[0] = 0;
  param_2[1] = 0;
  param_2[2] = 0;
  param_2[3] = 0;
  *(undefined2 *)(param_2 + 4) = 0;
  lVar1 = *(longlong *)(param_2 + 8);
  param_2[8] = 0;
  param_2[9] = 0;
  if (lVar1 != 0) {
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
  }
  *(code **)(param_2 + 6) = FUN_1403c7b10;
  param_2[10] = 0;
  param_2[0xb] = 0;
  (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1b0,uVar3);
  return param_1;
}

