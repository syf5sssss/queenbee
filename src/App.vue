<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import { listen } from '@tauri-apps/api/event';
const cmd = ref("");
const res = ref<Res[]>([]);

interface Res {
  ip: string;
  data: string[];
}

async function sheel() {
  if (cmd.value && cmd.value.length > 0) {
    await invoke("broadcast", { message: cmd.value });
  }
}
async function clear() {
  res.value = [];
}

const onResUpdated = (event: any) => {
  const data: Res = JSON.parse(event.payload);
  res.value?.push(data);
};

// 在组件挂载时开始监听事件
onMounted(async () => {
  await listen('res-updated', onResUpdated);
  await invoke('listen_udp');
});
</script>


<template>
  <div class="flex justify-content-evenly">

    <Button class="w-3 mt-1 ml-1" type="button" label="hello" @click="cmd = 'hello'"></Button>
    <Button class="w-3 mt-1 ml-1" type="button" label="ips:id" @click="cmd = 'ips:id'"></Button>
    <Button class="w-3 mt-1 ml-1" type="button" label="cmd:id:command" @click="cmd = 'cmd:id:command'"></Button>
    <Button class="w-3 mt-1 ml-1" type="button" label="info:id:str" @click="cmd = 'info:id:str'"></Button>
  </div>
  <div>
    <div class="flex start">
      <InputText class="w-full mt-1 " autocomplete="off" v-model="cmd" />
      <Button class="w-1 mt-1 ml-1" type="button" label="广播" @click="sheel()"></Button>
      <Button class="w-1 mt-1 ml-1" type="button" label="清空" @click="clear()"></Button>
    </div>
  </div>
  <div v-if="res">
    <div v-for="(item, index) in res" :key="index">
      <div>
        <p style="font-weight: bold; background-color:greenyellow;">{{ item.ip }}</p>
        <div v-for="(d, i) in item.data" :key="i">
          <p>{{ d }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped></style>