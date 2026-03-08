# Conda

## 说明

- 本页记录 `Conda` 的常用环境管理命令.
- 适合快速切换 Python 版本, 隔离依赖环境和导出环境配置.

## `conda activate` 指令无效

```sh
conda init powershell
```

执行后重启 PowerShell.

## 查看环境信息

### 列出当前环境中的 Python

```sh
conda list python
```

### 查看所有环境

```sh
conda info --envs
```

## 创建与切换环境

```sh
conda search python
conda create --name om6681 python=3.9.6
conda activate om6681
conda deactivate

conda create --name py310 python=3.10
conda activate py310
```

## 导出与删除环境

```sh
conda env export > environment.yml
conda remove -n py310 --all
```

## 使用建议

- 新项目优先为每个项目单独创建环境.
- 若系统里同时存在多个 Python 管理方案, 建议明确区分 `Conda` 和 `pyenv` 的职责.
- 团队协作时, 建议同时保留 `environment.yml` 方便复现.
- 如果环境越来越重, 可以定期清理不再使用的旧环境.
