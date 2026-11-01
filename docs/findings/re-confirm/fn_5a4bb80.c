
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_145a4bb80(undefined8 *param_1,int *param_2,undefined8 param_3,undefined8 param_4)

{
  int iVar1;
  undefined1 uVar2;
  int iVar3;
  int iVar4;
  
  *param_1 = FUN_1403c7b10;
  param_1[1] = 0;
  param_1[2] = 0;
  *(undefined8 *)((longlong)param_1 + 0x17) = 0;
  *(undefined8 *)((longlong)param_1 + 0x1f) = 0;
  *(undefined4 *)(param_1 + 5) = 1;
  uVar2 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                    (_DAT_15233c1b0,0x53,param_3,param_4,param_1 + 1);
  *(int *)(param_1 + 5) = param_2[4];
  iVar3 = param_2[1];
  iVar1 = param_2[2];
  iVar4 = param_2[3];
  *(int *)(param_1 + 3) = *param_2;
  *(int *)((longlong)param_1 + 0x1c) = iVar3;
  *(int *)(param_1 + 4) = iVar1;
  *(int *)((longlong)param_1 + 0x24) = iVar4;
  iVar3 = *param_2;
  iVar1 = param_2[1];
  if (iVar3 != 0 || iVar1 != 0) {
    if (param_2[2] - 0x75U < 0x1c) {
      iVar3 = FUN_145a490d0();
    }
    else {
      iVar4 = 0;
      if ((iVar3 != 0) && (iVar1 != 0)) {
        iVar4 = func_0x000145a4c660();
        iVar4 = iVar4 * iVar1 * iVar3;
      }
      if (*(char *)((longlong)param_2 + 0xd) == '\x01') {
        iVar3 = 6;
      }
      else {
        iVar3 = param_2[4];
      }
      iVar3 = iVar4 * iVar3 * ((uint)(*(char *)((longlong)param_2 + 0xd) == '\x03') * 5 + 1);
    }
    FUN_1403c7480(param_1,iVar3);
  }
  (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1b0,uVar2);
  return param_1;
}

