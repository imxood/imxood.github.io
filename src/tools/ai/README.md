# AI 工具总览

## 说明

- 本目录收录本地模型部署, OCR, RAG 和 AI 工作流相关记录.
- 若只是做日常模型接入或知识库实验, 可优先从本页进入.

## 本地模型与 Agent

- [Claude](./claude.md)
- [大模型工作流](./大模型工作流.md)
- [AutoGen Studio + Ollama 本地部署](./AutoGen_Ollama_本地部署.md)

## RAG 与知识库

- [RAGFlow](./ragflow.md)

## OCR 与文档解析

- [Chandra OCR](./chandra_ocr.md)

## 建议阅读路径

1. 想先把本地工具跑起来时, 先看 `Claude` 或 `AutoGen Studio + Ollama 本地部署`.
2. 想做知识库问答和文档检索时, 再看 `RAGFlow`.
3. 想做图片或文档 OCR 时, 优先看 `Chandra OCR`.
4. 想理解更上层的 Agent / 工作流组织方式, 再看 `大模型工作流`.

## 使用建议

- 如果目标是本地 Agent 或工作流编排, 先看 `AutoGen` 和 `大模型工作流`.
- 如果目标是知识库问答, 可优先看 `RAGFlow`.
- 如果目标是图片或文档识别, 可先看 `Chandra OCR`.
- 新增 AI 工具页时, 建议至少写清“适用场景, 依赖条件, 最小启动步骤, 常见坑点”.
