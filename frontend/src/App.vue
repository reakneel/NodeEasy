<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'

type Node = { id:string; name?:string; protocol:string; endpoint:{host:string;port:number}; status:string; score:number; last_tested_at?:string }
const nodes = ref<Node[]>([])
const loading = ref(true)
const online = ref(false)
const healthy = computed(() => nodes.value.filter(n => n.status === 'healthy').length)
async function load(){ loading.value=true; try { const r=await fetch('/api/v1/nodes'); if(!r.ok) throw Error(); nodes.value=(await r.json()).items } finally { loading.value=false } }
onMounted(async()=>{ await load(); try { const ws=new WebSocket(`${location.protocol==='https:'?'wss':'ws'}://${location.host}/api/v1/ws`); ws.onopen=()=>online.value=true; ws.onclose=()=>online.value=false } catch {} })
</script>
<template>
  <main class="min-h-screen bg-slate-950 text-slate-100 p-6 md:p-10">
    <header class="flex flex-wrap items-center justify-between gap-4 mb-8"><div><p class="text-cyan-400 text-sm font-semibold tracking-widest">NODEEASY V3</p><h1 class="text-3xl font-bold">节点数据中心</h1><p class="text-slate-400 mt-1">采集 · 标准化 · 测试 · 评分 · 分享</p></div><div class="rounded-full border border-slate-700 px-4 py-2 text-sm"><span :class="online?'text-emerald-400':'text-slate-500'">●</span> API {{online?'connected':'offline'}}</div></header>
    <section class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8"><article class="rounded-2xl bg-slate-900 border border-slate-800 p-5"><p class="text-slate-400">节点总数</p><strong class="text-4xl">{{nodes.length}}</strong></article><article class="rounded-2xl bg-slate-900 border border-slate-800 p-5"><p class="text-slate-400">健康节点</p><strong class="text-4xl text-emerald-400">{{healthy}}</strong></article><article class="rounded-2xl bg-slate-900 border border-slate-800 p-5"><p class="text-slate-400">平均评分</p><strong class="text-4xl">{{nodes.length ? (nodes.reduce((a,n)=>a+n.score,0)/nodes.length).toFixed(1) : '0.0'}}</strong></article></section>
    <section class="rounded-2xl bg-slate-900 border border-slate-800 overflow-hidden"><div class="px-5 py-4 border-b border-slate-800 flex justify-between"><h2 class="font-semibold">节点列表</h2><button class="text-cyan-400" @click="load">刷新</button></div><div v-if="loading" class="p-8 text-slate-400">Loading...</div><table v-else class="w-full text-sm"><thead class="text-slate-500"><tr><th class="text-left p-4">名称</th><th class="text-left p-4">协议</th><th class="text-left p-4">Endpoint</th><th class="text-left p-4">状态</th><th class="text-right p-4">评分</th></tr></thead><tbody><tr v-for="n in nodes" :key="n.id" class="border-t border-slate-800"><td class="p-4">{{n.name || 'Unnamed'}}</td><td class="p-4 uppercase">{{n.protocol}}</td><td class="p-4 font-mono">{{n.endpoint.host}}:{{n.endpoint.port}}</td><td class="p-4">{{n.status}}</td><td class="p-4 text-right">{{n.score.toFixed(1)}}</td></tr></tbody></table></section>
  </main>
</template>
