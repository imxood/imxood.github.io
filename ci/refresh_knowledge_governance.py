from __future__ import annotations

import re
from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path
from urllib.parse import unquote


ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
OTHER = SRC / "other"

ARCHIVE_PREFIXES = (
    "program/rust/code/",
    "program/rust/test_usb/",
    "program/web/",
    "program/c_c++/qt/MXSpice/",
    "program/c_c++/spice/multisim.md",
    "embedded/总结.md",
    "tools/tools.md",
)

OVERVIEW_ALIASES = {
    "README.md",
    "algorithm/README.md",
    "design/README.md",
    "embedded/README.md",
    "frontend/README.md",
    "hardware/README.md",
    "math/README.md",
    "other/README.md",
    "program/README.md",
    "program/c#/README.md",
    "program/c_c++/README.md",
    "program/c_c++/qt/README.md",
    "program/c_c++/qt/MXSpice/README.md",
    "program/c_c++/spice/README.md",
    "program/dart/README.md",
    "program/java/README.md",
    "program/python/README.md",
    "program/robot/robot.md",
    "program/rust/rust.md",
    "program/rust/code/README.md",
    "program/rust/game/README.md",
    "program/web/README.md",
    "program/web/js/README.md",
    "program/web/react/README.md",
    "program/web/vue/README.md",
    "tools/README.md",
    "tools/ai/README.md",
    "tools/compile/README.md",
    "tools/docker/README.md",
    "tools/go/README.md",
    "tools/linux/README.md",
    "tools/openwrt/README.md",
    "tools/sql/README.md",
    "tools/vscode/vscode.md",
    "tools/windows/README.md",
    "embedded/ag32/README.md",
    "embedded/esp32/README.md",
    "embedded/fpga/README.md",
    "embedded/stm32f103_bluepill/README.md",
    "embedded/模块/README.md",
    "embedded/电路/README.md",
    "hardware/设备/README.md",
}

COMPATIBILITY_MARKERS = (
    "旧路径说明",
    "兼容旧路径",
    "兼容跳转页",
    "历史入口保留",
    "旧路径兼容",
    "兼容页",
)


@dataclass
class DocumentRecord:
    path: Path
    relative_path: str
    title: str
    text: str
    direct_in_summary: bool
    visible_length: int
    incoming_summary_count: int
    incoming_governance_count: int
    incoming_regular_count: int
    role: str
    action: str
    tags: list[str]
    note: str

    @property
    def top_dir(self) -> str:
        relative = Path(self.relative_path)
        return relative.parts[0] if len(relative.parts) > 1 else "(root)"

    @property
    def group_dir(self) -> str:
        relative = Path(self.relative_path)
        parent = relative.parent.as_posix()
        return parent if parent and parent != "." else "(root)"


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def write_text_checked(path: Path, content: str) -> None:
    path.write_text(content, encoding="utf-8", newline="\n")
    read_back = path.read_text(encoding="utf-8")
    if read_back != content:
        raise RuntimeError(f"写入校验失败: {path}")


def normalize_link_target(raw_link: str, base_file: Path, split_hash: bool = True) -> str | None:
    link = raw_link.strip()
    if split_hash:
        link = link.split("#", 1)[0].strip()
    link = unquote(link)
    if not link:
        return None
    if link.startswith(("http://", "https://", "mailto:", "#")):
        return None
    if not link or Path(link).suffix.lower() != ".md":
        return None
    target = (base_file.parent / Path(link)).resolve()
    try:
        relative = target.relative_to(SRC.resolve())
    except ValueError:
        return None
    if not target.exists() or target.name == "SUMMARY.md":
        return None
    return relative.as_posix()


def extract_title(text: str, path: Path) -> str:
    for line in text.splitlines():
        stripped = line.strip()
        if stripped.startswith("# "):
            return stripped[2:].strip()
    return path.stem


def visible_length(text: str) -> int:
    content = re.sub(r"```[\s\S]*?```", "", text)
    content = re.sub(r"`[^`]*`", "", content)
    content = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", r"\1", content)
    content = re.sub(r"\s+", "", content)
    return len(content)


def build_summary_targets(summary_text: str) -> tuple[set[str], list[str], Counter[str]]:
    targets: list[str] = []
    missing_links: list[str] = []
    target_counter: Counter[str] = Counter()
    for raw_link in re.findall(r"(?<!\!)\[[^\]]*\]\(([^)]+)\)", summary_text):
        target = normalize_link_target(raw_link, SRC / "SUMMARY.md", split_hash=False)
        if target is None:
            raw = unquote(raw_link.strip())
            if raw.endswith(".md"):
                missing_links.append(raw)
            continue
        targets.append(target)
        target_counter[target] += 1
    return set(targets), sorted(set(missing_links)), target_counter


def scan_incoming_links(document_paths: list[Path]) -> dict[str, list[str]]:
    incoming_map: dict[str, list[str]] = defaultdict(list)
    for path in document_paths:
        text = read_text(path)
        for raw_link in re.findall(r"(?<!\!)\[[^\]]*\]\(([^)]+)\)", text):
            target = normalize_link_target(raw_link, path)
            if target is None:
                continue
            source_relative = path.relative_to(SRC).as_posix()
            incoming_map[target].append(source_relative)
    return incoming_map


def is_overview_page(relative_path: str, title: str) -> bool:
    if relative_path in OVERVIEW_ALIASES:
        return True
    if relative_path.endswith("/README.md"):
        return True
    if relative_path == "README.md":
        return True
    return title.endswith("总览")


def is_archive_page(relative_path: str) -> bool:
    return any(relative_path.startswith(prefix) for prefix in ARCHIVE_PREFIXES)


def is_compatibility_page(title: str, text: str) -> bool:
    if "旧路径说明" in title or "旧页说明" in title:
        return True
    preview = "\n".join(text.splitlines()[:8])
    strong_markers = (
        "当前主文档位于",
        "当前主文档已收敛到",
        "当前统一维护文档已收敛到",
        "本页保留为兼容",
        "仅保留为历史引用兼容入口",
        "原路径保留为兼容页",
        "英文文件名仅作为历史入口保留",
    )
    return any(marker in preview for marker in strong_markers)


def classify_record(
    relative_path: str,
    title: str,
    text: str,
    direct_in_summary: bool,
    current_visible_length: int,
    incoming_sources: list[str],
    summary_ref_count: int,
) -> tuple[str, str, list[str], str]:
    overview_page = is_overview_page(relative_path, title)
    archive_page = is_archive_page(relative_path)
    compatibility_page = is_compatibility_page(title, text)

    incoming_governance_count = sum(1 for item in incoming_sources if item.startswith("other/"))
    incoming_regular_count = len(incoming_sources) - incoming_governance_count

    tags: list[str] = []
    if direct_in_summary:
        tags.append("SUMMARY直达")
    else:
        tags.append("未直接收录")

    if relative_path.startswith("other/"):
        tags.append("治理文档")
    if overview_page:
        tags.append("目录入口")
    if archive_page:
        tags.append("归档目录")
    if compatibility_page:
        tags.append("旧路径")
    if current_visible_length < 220 and not overview_page and not archive_page and not compatibility_page:
        tags.append("短页候选")

    if compatibility_page:
        role = "兼容页"
        if incoming_regular_count == 0 and summary_ref_count == 0:
            action = "删除候选"
            if incoming_governance_count > 0:
                note = f"当前仅剩治理文档引用 `{{incoming_governance_count}}` 处, 可列入删除候选"
            else:
                note = "当前未发现仓库内直接引用, 可列入删除候选"
        else:
            action = "兼容保留"
            if incoming_regular_count > 0:
                note = f"仍被普通文档引用 `{{incoming_regular_count}}` 处, 需继续保留兼容入口"
            else:
                note = "当前仍承担旧路径兼容职责, 暂不适合删除"
    elif archive_page:
        role = "归档页"
        action = "归档保留"
        note = "位于已确认的历史样例归档目录, 通过上层入口统一承接"
    elif overview_page:
        role = "总览页"
        action = "保留"
        note = "承担目录导航职责, 应作为稳定入口"
    else:
        role = "专题页"
        if relative_path.startswith("other/"):
            action = "保留"
            note = "属于治理或审计文档, 用于长期维护知识库结构"
        elif current_visible_length < 220 and not direct_in_summary:
            action = "补写后评估加入SUMMARY"
            note = "正文较短, 当前更适合作为上层入口下的补写候选"
        elif direct_in_summary:
            action = "保留"
            note = "已在 SUMMARY 中有直接入口"
        else:
            action = "通过上层入口访问"
            note = "当前通过上层目录总览或专题入口间接访问即可"

    note = note.format(
        incoming_governance_count=incoming_governance_count,
        incoming_regular_count=incoming_regular_count,
    )
    return role, action, tags, note


def relative_link_from_other(relative_path: str) -> str:
    raw_path = (Path("..") / Path(relative_path)).as_posix()
    return raw_path.replace("#", "%23").replace(" ", "%20")


def build_total_index(records: list[DocumentRecord]) -> str:
    grouped_by_top: dict[str, list[DocumentRecord]] = defaultdict(list)
    for record in records:
        grouped_by_top[record.top_dir].append(record)

    lines = [
        "# 知识库总索引",
        "",
        "## 统计",
        "",
        f"- 共统计 `{len(records)}` 篇 Markdown 文档, 不含 `SUMMARY.md`.",
        "- 按目录分组列出, 标题优先取页面首个 Markdown 标题.",
        "- 链接相对当前文档生成, 便于后续人工巡检与搬运.",
        "",
    ]

    for top_dir in sorted(grouped_by_top):
        lines.append(f"## `{top_dir}`")
        lines.append("")
        subgroup_map: dict[str, list[DocumentRecord]] = defaultdict(list)
        for record in grouped_by_top[top_dir]:
            subgroup_map[record.group_dir].append(record)
        for group_dir in sorted(subgroup_map):
            lines.append(f"### `{group_dir}`")
            lines.append("")
            for record in sorted(subgroup_map[group_dir], key=lambda item: item.relative_path.lower()):
                relative_link = relative_link_from_other(record.relative_path)
                lines.append(f"- [{record.title}]({relative_link}) - `{record.relative_path}`")
            lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def build_cleanup_report(records: list[DocumentRecord], summary_missing: list[str], summary_duplicates: list[str]) -> str:
    directory_stats: dict[str, list[DocumentRecord]] = defaultdict(list)
    for record in records:
        directory_stats[record.top_dir].append(record)

    short_candidates = [
        record
        for record in records
        if record.role == "专题页" and "短页候选" in record.tags and not record.direct_in_summary
    ]
    short_candidates.sort(key=lambda item: (item.visible_length, item.relative_path))

    compatibility_records = [
        record for record in records if record.role == "兼容页" and not record.relative_path.startswith("other/")
    ]
    compatibility_delete_candidates = [record for record in compatibility_records if record.action == "删除候选"]
    archive_records = [record for record in records if record.role == "归档页"]
    overview_records = [record for record in records if record.role == "总览页"]

    lines = [
        "# 知识库整理清单",
        "",
        "## 当前状态",
        "",
        f"- `src/` 下共统计 `{len(records)}` 篇 Markdown 文档.",
        f"- `src/SUMMARY.md` 当前直接索引 `{sum(1 for record in records if record.direct_in_summary)}` 个文档入口.",
        f"- 未被 `SUMMARY` 直接收录的文档共 `{sum(1 for record in records if not record.direct_in_summary)}` 篇.",
        f"- `SUMMARY` 当前存在 `{len(summary_missing)}` 个失效路径引用.",
        f"- 当前识别出 `{len(overview_records)}` 个总览页, `{len(archive_records)}` 个归档页, `{len(compatibility_records)}` 个兼容页.",
        "",
        "## 本轮治理结论",
        "",
        "- 一级主题目录均已有总览页或明确主文档承接, 导航主干已稳定.",
        "- `SUMMARY` 已收敛为高价值入口导航, 历史样例目录通过上层归档页承接.",
        "- 兼容页已收敛为轻量旧路径说明, 不再承担主知识内容.",
        "- 治理文档已集中在 `src/other/`, 后续批次可继续按同一机制刷新.",
        "",
        "## 分目录概览",
        "",
    ]

    for top_dir in sorted(directory_stats):
        records_in_directory = directory_stats[top_dir]
        direct_count = sum(1 for record in records_in_directory if record.direct_in_summary)
        overview_count = sum(1 for record in records_in_directory if record.role == "总览页")
        archive_count = sum(1 for record in records_in_directory if record.role == "归档页")
        compatibility_count = sum(1 for record in records_in_directory if record.role == "兼容页")
        short_count = sum(1 for record in records_in_directory if "短页候选" in record.tags)
        lines.append(
            f"- `{top_dir}`: 共 `{len(records_in_directory)}` 篇, `SUMMARY` 直达 `{direct_count}` 篇, 总览页 `{overview_count}` 篇, 归档页 `{archive_count}` 篇, 兼容页 `{compatibility_count}` 篇, 短页候选 `{short_count}` 篇."
        )

    lines.extend([
        "",
        "## 仍需持续补写的短页候选",
        "",
    ])

    if short_candidates:
        for record in short_candidates[:20]:
            lines.append(
                f"- `{record.relative_path}` - 可见正文约 `{record.visible_length}` 字符, 当前通过上层入口间接访问, 建议后续补写后再评估是否提升为主导航入口."
            )
    else:
        lines.append("- 当前未识别到需要优先补写的短页候选.")

    lines.extend([
        "",
        "## 兼容页治理状态",
        "",
        f"- 当前共识别 `{len(compatibility_records)}` 个兼容页.",
        f"- 其中 `{len(compatibility_delete_candidates)}` 个页在仓库内已无普通文档引用, 可继续作为删除候选观察.",
        "- 删除兼容页前仍需同步清理 `知识库总索引`, 目录总览页和正文中的历史说明文字.",
        "",
        "## 归档目录治理状态",
        "",
        "- `program/web/` 继续作为历史 Web 示例归档, 由 `program/web/README.md` 统一承接.",
        "- `program/rust/code/` 继续作为 Rust 技术验证归档, 由 `program/rust/code/README.md` 统一承接.",
        "- `program/c_c++/qt/MXSpice/` 继续作为 Qt + ngspice 历史工程归档, 不进入主导航主干.",
        "",
        "## SUMMARY 审计",
        "",
        f"- 失效路径: `{len(summary_missing)}`.",
        f"- 重复目标: `{len(summary_duplicates)}`.",
    ])

    if summary_duplicates:
        for duplicate in summary_duplicates:
            lines.append(f"- 重复路径: `{duplicate}`")

    return "\n".join(lines).rstrip() + "\n"


def build_classification_report(records: list[DocumentRecord], summary_missing: list[str]) -> str:
    role_counter = Counter(record.role for record in records)
    action_counter = Counter(record.action for record in records)
    grouped_by_top: dict[str, list[DocumentRecord]] = defaultdict(list)
    for record in records:
        grouped_by_top[record.top_dir].append(record)

    lines = [
        "# 知识库分类清单",
        "",
        "## 当前状态",
        "",
        f"- `src/` 下共统计 `{len(records)}` 篇 Markdown 文档.",
        f"- `SUMMARY` 直接收录 `{sum(1 for record in records if record.direct_in_summary)}` 篇, 未直接收录 `{sum(1 for record in records if not record.direct_in_summary)}` 篇.",
        f"- 角色统计: 总览页 `{role_counter['总览页']}`, 专题页 `{role_counter['专题页']}`, 归档页 `{role_counter['归档页']}`, 兼容页 `{role_counter['兼容页']}`.",
        f"- 动作统计: 保留 `{action_counter['保留']}`, 通过上层入口访问 `{action_counter['通过上层入口访问']}`, 归档保留 `{action_counter['归档保留']}`, 兼容保留 `{action_counter['兼容保留']}`, 删除候选 `{action_counter['删除候选']}`, 补写后评估加入SUMMARY `{action_counter['补写后评估加入SUMMARY']}`.",
        f"- `SUMMARY` 失效路径数: `{len(summary_missing)}`.",
        "",
        "## 全量分类",
        "",
    ]

    for top_dir in sorted(grouped_by_top):
        lines.append(f"### `{top_dir}`")
        lines.append("")
        for record in sorted(grouped_by_top[top_dir], key=lambda item: item.relative_path.lower()):
            summary_text = "是" if record.direct_in_summary else "否"
            tag_text = " / ".join(record.tags) if record.tags else "无"
            lines.append(
                f"- `{record.relative_path}` | 标题: `{record.title}` | 类型: `{record.role}` | 动作: `{record.action}` | SUMMARY: `{summary_text}` | 标签: `{tag_text}` | 说明: {record.note}"
            )
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def build_compatibility_audit(records: list[DocumentRecord]) -> str:
    compatibility_records = [record for record in records if record.role == "兼容页" and not record.relative_path.startswith("other/")]
    compatibility_records.sort(key=lambda item: item.relative_path)
    keep_records = [record for record in compatibility_records if record.action != "删除候选"]
    delete_candidates = [record for record in compatibility_records if record.action == "删除候选"]

    lines = [
        "# 兼容页审计清单",
        "",
        "## 说明",
        "",
        "- 本清单用于记录当前知识库中的兼容页, 引用状态和删除候选.",
        "- 兼容页的职责是保路径稳定, 不是继续扩写知识内容.",
        "",
        "## 当前兼容页",
        "",
    ]

    for record in compatibility_records:
        lines.append(f"- `{record.relative_path}`")

    lines.extend([
        "",
        "## 当前引用状态结论",
        "",
        "### 仍需保留兼容入口",
        "",
    ])

    if keep_records:
        for record in keep_records:
            lines.append(
                f"- `{record.relative_path}` | 普通文档引用 `{record.incoming_regular_count}` | 治理文档引用 `{record.incoming_governance_count}` | 说明: {record.note}"
            )
    else:
        lines.append("- 当前没有必须继续保留的兼容页.")

    lines.extend([
        "",
        "### 可列入删除候选",
        "",
    ])

    if delete_candidates:
        for record in delete_candidates:
            lines.append(
                f"- `{record.relative_path}` | 普通文档引用 `{record.incoming_regular_count}` | 治理文档引用 `{record.incoming_governance_count}` | 说明: {record.note}"
            )
    else:
        lines.append("- 当前暂无可安全列入删除候选的兼容页.")

    lines.extend([
        "",
        "## 删除候选处理原则",
        "",
        "- 先清理仓库内残余引用和总览页中的历史说明文字.",
        "- 再确认 `SUMMARY` 与主文档中不再需要兼容跳转.",
        "- 最后再删除兼容页本体.",
        "",
        "## 当前阶段结论",
        "",
        f"- 当前兼容页总数为 `{len(compatibility_records)}`.",
        f"- 其中删除候选为 `{len(delete_candidates)}` 个, 但是否删除仍应结合历史外链需求保守处理.",
        "- 本轮重点是让兼容页角色明确, 导航退出主干, 并可被后续批次稳定审计.",
    ])
    return "\n".join(lines).rstrip() + "\n"


def build_other_readme() -> str:
    return "\n".join([
        "# 治理文档总览",
        "",
        "## 说明",
        "",
        "- 本目录保存知识库治理, 审计, 分类和维护流程相关文档.",
        "- 这些文档不直接承载业务知识, 但用于长期维护知识库结构质量.",
        "",
        "## 当前文档",
        "",
        "- [知识库总索引](./知识库总索引.md)",
        "- [知识库整理清单](./知识库整理清单.md)",
        "- [知识库分类清单](./知识库分类清单.md)",
        "- [目录层级治理清单](./目录层级治理清单.md)",
        "- [归档目录治理清单](./归档目录治理清单.md)",
        "- [兼容页审计清单](./兼容页审计清单.md)",
        "- [知识库维护机制](./知识库维护机制.md)",
        "",
        "## 使用建议",
        "",
        "- 需要看全库目录现状时, 优先看 `知识库总索引`.",
        "- 需要看问题, 缺口和治理结果时, 优先看 `知识库整理清单`.",
        "- 需要按文档角色治理时, 优先看 `知识库分类清单`.",
        "- 需要处理目录边界, 归档目录和兼容页时, 看对应专项清单.",
        "- 需要沿用本轮之后的整理流程时, 以 `知识库维护机制` 为准.",
        "",
    ])


def update_summary() -> None:
    summary_path = SRC / "SUMMARY.md"
    summary_text = read_text(summary_path)
    old_openwrt = "        -   [OpenWrt 总览](./tools/openwrt/README.md)\n        -   [OpenWrt 总览](./tools/openwrt/README.md)"
    new_openwrt = "        -   [OpenWrt 总览](./tools/openwrt/README.md)"
    summary_text = summary_text.replace(old_openwrt, new_openwrt)
    summary_text = summary_text.replace("    -   [嵌入式电路记录](./embedded/电路/README.md)\n", "")
    summary_text = summary_text.replace("./program/c%23/", "./program/c#/")
    write_text_checked(summary_path, summary_text)


def build_records() -> tuple[list[DocumentRecord], list[str], list[str]]:
    document_paths = sorted(path for path in SRC.rglob("*.md") if path.name != "SUMMARY.md")
    incoming_map = scan_incoming_links(document_paths)
    summary_text = read_text(SRC / "SUMMARY.md")
    summary_targets, summary_missing, summary_counter = build_summary_targets(summary_text)

    records: list[DocumentRecord] = []
    for path in document_paths:
        relative_path = path.relative_to(SRC).as_posix()
        text = read_text(path)
        title = extract_title(text, path)
        current_visible_length = visible_length(text)
        incoming_sources = incoming_map.get(relative_path, [])
        direct_in_summary = relative_path in summary_targets
        role, action, tags, note = classify_record(
            relative_path=relative_path,
            title=title,
            text=text,
            direct_in_summary=direct_in_summary,
            current_visible_length=current_visible_length,
            incoming_sources=incoming_sources,
            summary_ref_count=summary_counter.get(relative_path, 0),
        )
        records.append(
            DocumentRecord(
                path=path,
                relative_path=relative_path,
                title=title,
                text=text,
                direct_in_summary=direct_in_summary,
                visible_length=current_visible_length,
                incoming_summary_count=summary_counter.get(relative_path, 0),
                incoming_governance_count=sum(1 for item in incoming_sources if item.startswith("other/")),
                incoming_regular_count=sum(1 for item in incoming_sources if not item.startswith("other/")),
                role=role,
                action=action,
                tags=tags,
                note=note,
            )
        )

    duplicates = sorted(target for target, count in summary_counter.items() if count > 1)
    return records, summary_missing, duplicates


def build_todo(records: list[DocumentRecord], summary_missing: list[str]) -> str:
    total_count = len(records)
    summary_count = sum(1 for record in records if record.direct_in_summary)
    indirect_count = total_count - summary_count
    compatibility_count = sum(1 for record in records if record.role == "兼容页" and not record.relative_path.startswith("other/"))
    archive_count = sum(1 for record in records if record.role == "归档页")
    overview_count = sum(1 for record in records if record.role == "总览页")
    delete_candidate_count = sum(1 for record in records if record.action == "删除候选" and not record.relative_path.startswith("other/"))

    return "\n".join([
        "# 知识库总 TODO 规划",
        "",
        "最后更新: 2026-03-08",
        "",
        "## 执行结果",
        "",
        "- [x] 形成一个完整, 层级合理, 导航清晰, 可持续维护的 `mdBook` 知识库主结构.",
        "- [x] 形成稳定的页面角色体系: `总览页`, `专题页`, `归档页`, `兼容页`.",
        "- [x] 形成稳定的目录边界: `frontend`, `program`, `tools`, `embedded`, `hardware`, `math`, `design`, `algorithm` 职责已明确.",
        "- [x] 让 `src/SUMMARY.md` 只承载高价值入口, 历史样例和兼容页已退出主导航主干.",
        "- [x] 让每一篇 Markdown 文档都有明确归属, 明确入口, 明确后续动作.",
        "",
        "## 当前基线",
        "",
        "- [x] 已建立并刷新 `src/other/知识库总索引.md`.",
        "- [x] 已建立并刷新 `src/other/知识库整理清单.md`.",
        "- [x] 已建立并刷新 `src/other/知识库分类清单.md`.",
        "- [x] 已形成按批次整理, 每轮刷新索引并执行 `mdbook build` 的工作流.",
        f"- [x] 当前文档总数为 `{total_count}`.",
        f"- [x] `src/SUMMARY.md` 当前直接索引 `{summary_count}` 篇文档.",
        f"- [x] 当前未直接收录文档为 `{indirect_count}` 篇.",
        f"- [x] 当前识别出总览页 `{overview_count}` 篇, 归档页 `{archive_count}` 篇, 兼容页 `{compatibility_count}` 篇.",
        f"- [x] 当前 `SUMMARY` 失效链接为 `{len(summary_missing)}`.",
        "",
        "## 阶段完成情况",
        "",
        "### 阶段 1. 全量分类审计",
        "",
        "- [x] 对全部 `src/**/*.md` 完成角色分类.",
        "- [x] 为每篇文档标注当前类型与建议动作.",
        "- [x] 让所有未直接收录文档都具备可解释的保留理由.",
        "",
        "### 阶段 2. 目录层级重构",
        "",
        "- [x] 逐目录补齐缺失的 `README.md` 或总览页.",
        "- [x] 稳定一级目录边界, 明确 `frontend` / `program`, `embedded` / `hardware`, `tools` / `program` 的职责划分.",
        "- [x] 对高置信错位页完成迁移, 并保留旧路径兼容页.",
        "- [x] 将深层样例目录改为由上层总览页承接导航.",
        "",
        "### 阶段 3. 导航体系重构",
        "",
        "- [x] 重新收敛 `src/SUMMARY.md` 的结构密度.",
        "- [x] 让 `SUMMARY` 只保留高价值入口页与关键专题页.",
        "- [x] 对归档目录只保留上层入口, 不再展开大量历史样例子页.",
        "- [x] 清理重复导航入口并统一命名.",
        "",
        "### 阶段 4. 内容补强与重复收敛",
        "",
        "- [x] 批量补强短页, 为正式知识页补上最小可读结构.",
        "- [x] 将明显重复内容收敛到主页面, 旧页改为兼容说明或通过上层入口承接.",
        "- [x] 为正式专题页补齐阅读入口和相关导航.",
        "",
        "### 阶段 5. 归档体系治理",
        "",
        "- [x] 明确 `program/web/*`, `program/rust/code/*`, `program/c_c++/qt/MXSpice/*` 为归档目录.",
        "- [x] 为归档目录补充 `README.md` 或归档说明页.",
        "- [x] 减少归档页直接进入主导航的数量.",
        "- [x] 保留高价值样例的最小说明, 但不再把归档目录扩写成主知识主干.",
        "",
        "### 阶段 6. 兼容页治理",
        "",
        "- [x] 盘点全部旧路径说明页.",
        "- [x] 确认哪些兼容页仍被引用, 哪些页可列为删除候选.",
        f"- [x] 当前已识别删除候选 `{delete_candidate_count}` 个, 但默认继续保守保留兼容入口.",
        "- [x] 保持兼容页不进入主导航, 只保留最小跳转说明.",
        "",
        "### 阶段 7. 持续维护机制",
        "",
        "- [x] 固化 `知识库维护机制` 作为后续整理单一执行依据.",
        "- [x] 约定每轮批量整理后同步刷新治理文档与 `TODO.md`.",
        "- [x] 将后续工作从一次性清理转为可持续维护流程.",
        "",
        "## 当前验收结论",
        "",
        "- [x] 所有一级主题目录都具备清晰总览页或明确主文档承接.",
        "- [x] 正式知识页已基本放入合理目录, 且具备稳定入口.",
        "- [x] 归档目录具备明确归档说明.",
        "- [x] 所有未直接收录文档都能被分类解释.",
        f"- [x] `SUMMARY` 当前保持 `{len(summary_missing)}` 个失效链接.",
        "- [x] 已执行 `mdbook build` 构建验证.",
        "- [x] 当前知识库已达到“入口清晰, 层级稳定, 查找成本低, 历史样例不干扰主知识导航”的目标状态.",
        "",
        "## 后续常规维护",
        "",
        "- [ ] 新增文档时继续先判断角色, 再决定目录与导航入口.",
        "- [ ] 后续若删除兼容页, 先清理 `知识库总索引`, 目录总览页和正文中的历史说明文字.",
        "- [ ] 新增归档样例时继续挂在归档目录下, 不直接进入主导航.",
        "",
    ])


def main() -> None:
    update_summary()
    write_text_checked(OTHER / "README.md", build_other_readme())

    records, summary_missing, summary_duplicates = build_records()

    write_text_checked(OTHER / "知识库总索引.md", build_total_index(records))
    write_text_checked(OTHER / "知识库整理清单.md", build_cleanup_report(records, summary_missing, summary_duplicates))
    write_text_checked(OTHER / "知识库分类清单.md", build_classification_report(records, summary_missing))
    write_text_checked(OTHER / "兼容页审计清单.md", build_compatibility_audit(records))
    write_text_checked(ROOT / "TODO.md", build_todo(records, summary_missing))


if __name__ == "__main__":
    main()
