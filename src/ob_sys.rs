use wasm_bindgen::prelude::*;
use js_sys::{ Promise, Object, Array, ArrayBuffer, Function };
use web_sys::{ HtmlElement };

// low-level APIs (plumbing, not porcelain; intentionally brutalist; eventually I want this file autogenned)
// note we preserve camelCase here; contextual cue that you're dealing with import
// TODO: build thing to turn typescript.d.ts files into imports via AST

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    pub type AbstractTextComponent;
    #[wasm_bindgen(extends = Events)]
    pub type App;
    #[wasm_bindgen(extends = Object)]
    pub type BaseComponent;
    #[wasm_bindgen(extends = Object)]
    pub type BlockCache;
    #[wasm_bindgen(extends = SubpathResult)]
    pub type BlockSubpathResult;
    #[wasm_bindgen(extends = BaseComponent)]
    pub type ButtonComponent;
    #[wasm_bindgen(extends = Object)]
    pub type CachedMetadata;
    #[wasm_bindgen(extends = Object)]
    pub type Command;
    #[wasm_bindgen(js_name = Component, extends = Events)]
    pub type Component;
    #[wasm_bindgen(extends = Object)]
    pub type DataAdapter;
    #[wasm_bindgen(extends = ValueComponent)]
    pub type DropdownComponent;
    #[wasm_bindgen(extends = FileView)]
    pub type EditableFileView;
    #[wasm_bindgen(extends = ReferenceCache)]
    pub type EmbedCache;
    #[wasm_bindgen(extends = Object)]
    pub type Events;
    #[wasm_bindgen(extends = Object)]
    pub type EventRef;
    #[wasm_bindgen(extends = BaseComponent)]
    pub type ExtraButtonComponent;
    #[wasm_bindgen(extends = Object)]
    pub type FileStats;
    #[wasm_bindgen(extends = DataAdapter)] // recall typescript interfaces are treated kinda like JS objects by bindgen
    pub type FileSystemAdapter;
    #[wasm_bindgen(extends = ItemView)]
    pub type FileView;
    #[wasm_bindgen(extends = Object)]
    pub type FrontMatterCache;
    #[wasm_bindgen(extends = Object)]
    pub type HeadingCache;
    #[wasm_bindgen(extends = SubpathResult)]
    pub type HeadingSubpathResult;
    #[wasm_bindgen(extends = Object)]
    pub type HoverParent;
    #[wasm_bindgen(extends = Component)]
    pub type HoverPopover;
    #[wasm_bindgen(extends = View)]
    pub type ItemView;
    #[wasm_bindgen(extends = Object)]
    pub type KeymapEventHandler;
    #[wasm_bindgen(extends = Object)]
    pub type KeymapEventListener;
    #[wasm_bindgen(extends = ReferenceCache)]
    pub type LinkCache;
    #[wasm_bindgen(extends = Object)]
    pub type ListedFiles;
    #[wasm_bindgen(extends = Object)]
    pub type Loc;
    #[wasm_bindgen(extends = Object)]
    pub type MarkdownPostProcessor;
    #[wasm_bindgen(extends = Object)]
    pub type MarkdownPostProcessorContext;
    #[wasm_bindgen(extends = Object)]
    pub type MarkdownPreviewEvents;
    #[wasm_bindgen(extends = Object)]
    pub type MarkdownPreviewRenderer;
    #[wasm_bindgen(extends = MarkdownRenderer, extends = MarkdownSubView, extends = MarkdownPreviewEvents)]
    pub type MarkdownPreviewView;
    #[wasm_bindgen(extends = Component, extends = MarkdownPreviewEvents, extends = HoverParent)]
    pub type MarkdownRenderer;
    #[wasm_bindgen(extends = MarkdownSubView, extends = HoverParent)]
    pub type MarkdownSourceView;
    #[wasm_bindgen(extends = Object)]
    pub type MarkdownSubView;
    #[wasm_bindgen(extends = EditableFileView)]
    pub type MarkdownView;
    #[wasm_bindgen(extends = Object)]
    pub type Menu;
    #[wasm_bindgen(extends = Object)]
    pub type Modal;
    #[wasm_bindgen(extends = Object)]
    pub type Notice;
    #[wasm_bindgen(extends = Object)]
    pub type OpenViewState;
    #[wasm_bindgen(extends = Component, extends = Events)]
    pub type Plugin;
    #[wasm_bindgen(extends = Object)]
    pub type PluginSettingTab;
    #[wasm_bindgen(extends = Object)]
    pub type PluginManifest;
    #[wasm_bindgen(extends = Object)]
    pub type Pos;
    #[wasm_bindgen(extends = Object)]
    pub type Point;
    #[wasm_bindgen(extends = Object)]
    pub type PopoverState;
    #[wasm_bindgen(extends = Object)]
    pub type Rect;
    #[wasm_bindgen(extends = Object)]
    pub type ReferenceCache;
    #[wasm_bindgen(extends = Object)]
    pub type Scope;
    #[wasm_bindgen(extends = Object)]
    pub type Setting;
    #[wasm_bindgen(extends = Object)]
    pub type SettingTab;
    #[wasm_bindgen(extends = ValueComponent)]
    pub type SliderComponent;
    #[wasm_bindgen(extends = Object)]
    pub type SplitDirection;
    #[wasm_bindgen(extends = Object)]
    pub type SubpathResult;
    #[wasm_bindgen(extends = Object)]
    pub type TagCache;
    #[wasm_bindgen(extends = Object)]
    pub type TAbstractFile;
    #[wasm_bindgen(extends = AbstractTextComponent)]
    pub type TextAreaComponent;
    #[wasm_bindgen(extends = AbstractTextComponent)]
    pub type TextComponent;
    #[wasm_bindgen(extends = Object)]
    pub type TFile;
    #[wasm_bindgen(extends = Object)]
    pub type TFolder;
    #[wasm_bindgen(extends = BaseComponent)]
    pub type ToggleComponent;
    #[wasm_bindgen(extends = BaseComponent)]
    pub type ValueComponent;
    #[wasm_bindgen(extends = Component)]
    pub type Vault;
    #[wasm_bindgen(extends = Component)]
    pub type View;
    #[wasm_bindgen(extends = Object)]
    pub type ViewCreator;
    #[wasm_bindgen(extends = Object)]
    pub type ViewState;
    #[wasm_bindgen(extends = Object)]
    pub type ViewStateResult;
    #[wasm_bindgen(extends = Events)]
    pub type Workspace;
    #[wasm_bindgen(extends = Object)]
    pub type WorkspaceLeaf;
    #[wasm_bindgen(extends = Object)]
    pub type WorkspaceItem;
    #[wasm_bindgen(extends = Object)]
    pub type WorkspaceRibbon;
    #[wasm_bindgen(extends = Object)]
    pub type WorkspaceSidedock;
    #[wasm_bindgen(extends = Object)]
    pub type WorkspaceSplit;
    #[wasm_bindgen(extends = Object)]
    pub type WorkspaceTabs;

    #[wasm_bindgen(method, js_class = "App")]
    pub fn App_on(app: &App, eventName: &str, callback: Function, ctx: JsValue);

    #[wasm_bindgen(method, js_class = "Events")]
    pub fn Events_on(events: &Events, eventName: &str);

    #[wasm_bindgen(method, js_class = "Component", js_name = load)]
    pub fn Component_load(this: &Component);
    #[wasm_bindgen(method, js_class = "Component", js_name = unload)]
    pub fn Component_unload(this: &Component);
    #[wasm_bindgen(method, js_class = "Component", js_name = addChild)]
    pub fn Component_addChild(this: &Component, child: &Component);
    #[wasm_bindgen(method, js_class = "Component", js_name = removeChild)]
    pub fn Component_removeChild(this: &Component, child: &Component) -> bool;

    #[wasm_bindgen(method, js_class = "Plugin", js_name = addRibbonIcon)]
    pub fn Plugin_addRibbonIcon(this: &Plugin, icon: &str, title: &str) -> HtmlElement;
    #[wasm_bindgen(method, js_class = "Plugin", js_name = addStatusBarItem)]
    pub fn Plugin_addStatusBarItem(this: &Plugin) -> HtmlElement;
    #[wasm_bindgen(method, js_class = "Plugin", js_name = addCommand)]
    pub fn Plugin_addCommand(this: &Plugin, command: &Command);
    #[wasm_bindgen(method, js_class = "Plugin", js_name = addSettingTab)]
    pub fn Plugin_addSettingTab(this: &Plugin, settingTab: &PluginSettingTab);
    #[wasm_bindgen(method, js_class = "Plugin", js_name = registerView)]
    pub fn Plugin_registerView(this: &Plugin, viewCreator: &ViewCreator);
    #[wasm_bindgen(method, js_class = "Plugin", js_name = registerExtensions)]
    pub fn Plugin_registerExtensions(this: &Plugin, extensions: &Array, viewType: &str);
    #[wasm_bindgen(method, js_class = "Plugin", js_name = loadData)]
    pub fn Plugin_loadData(this: &Plugin) -> Promise;
    #[wasm_bindgen(method, js_class = "Plugin", js_name = saveData)]
    pub fn Plugin_saveData(this: &Plugin, data: &Object) -> Promise;

    #[wasm_bindgen(method, js_class = "Vault", js_name = getName)]
    pub fn Vault_getName(this: &Plugin) -> String;
    #[wasm_bindgen(method, js_class = "Vault", js_name = getAbstractFileByPath)]
    pub fn Vault_getAbstractFileByPath(this: &Vault, path: &str) -> TAbstractFile;
    #[wasm_bindgen(method, js_class = "Vault", js_name = getRoot)]
    pub fn Vault_getRoot(this: &Vault) -> TFolder;
    #[wasm_bindgen(method, js_class = "Vault", js_name = create)]
    pub fn Vault_create(this: &Vault, path: &str, data: &str) -> Promise;
    #[wasm_bindgen(method, js_class = "Vault", js_name = createBinary)]
    pub fn Vault_createBinary(this: &Vault, path: &str, data: &ArrayBuffer) -> Promise;
    #[wasm_bindgen(method, js_class = "Vault", js_name = createFolder)]
    pub fn Vault_createFolder(this: &Vault, path: &str) -> Promise;
    #[wasm_bindgen(method, js_class = "Vault", js_name = read)]
    pub fn Vault_read(this: &Vault, file: &TFile) -> Promise;
    #[wasm_bindgen(method, js_class = "Vault", js_name = cachedRead)]
    pub fn Vault_cachedRead(this: &Vault, file: &TFile) -> Promise;
    #[wasm_bindgen(method, js_class = "Vault", js_name = readBinary)]
    pub fn Vault_readBinary(this: &Vault, file: &ArrayBuffer) -> Promise;
    #[wasm_bindgen(method, js_class = "Vault", js_name = getResourcePath)]
    pub fn Vault_getResourcePath(this: &Vault, file: &TFile) -> Promise;
    #[wasm_bindgen(method, js_class = "Vault", js_name = delete)]
    pub fn Vault_delete(this: &Vault) -> Promise;
    #[wasm_bindgen(method, js_class = "Vault", js_name = trash)]
    pub fn Vault_trash(this: &Vault, file: &TAbstractFile, system: bool) -> Promise;
    #[wasm_bindgen(method, js_class = "Vault", js_name = rename)]
    pub fn Vault_rename(this: &Vault, file: &TFile, newPath: &str) -> Promise;
    #[wasm_bindgen(method, js_class = "Vault", js_name = modify)]
    pub fn Vault_modify(this: &Vault, file: &TFile, data: &str);
    #[wasm_bindgen(method, js_class = "Vault", js_name = modifyBinary)]
    pub fn Vault_modifyBinary(this: &Vault, file: &TFile, data: &ArrayBuffer);
    #[wasm_bindgen(method, js_class = "Vault", js_name = copy)]
    pub fn Vault_copy(this: &Vault, file: &TFile, newPath: &str) -> Promise;
    #[wasm_bindgen(method, js_class = "Vault", js_name = getAllLoadedFiles)]
    pub fn Vault_getAllLoadedFiles(this: &Vault) -> Array;
    #[wasm_bindgen(method, js_class = "Vault", js_name = recurseChildren)]
    pub fn Vault_recurseChildren(this: &Vault, root: &TFolder, cb: &Function);
    #[wasm_bindgen(method, js_class = "Vault", js_name = on)]
    pub fn Vault_on(this: &Vault, name: &str, callback: &Function, context: JsValue) -> EventRef;

    #[wasm_bindgen(constructor, js_class = "View")]
    pub fn View_new(leaf: &WorkspaceLeaf) -> View;
    #[wasm_bindgen(method, js_class = "View", js_name = onOpen)]
    pub fn View_onOpen(this: &View) -> Promise;
    #[wasm_bindgen(method, js_class = "View", js_name = onClose)]
    pub fn View_onClose(this: &View) -> Promise;
    #[wasm_bindgen(method, js_class = "View", js_name = getViewType)]
    pub fn View_getViewType(this: &View) -> String;
    #[wasm_bindgen(method, js_class = "View", js_name = getState)]
    pub fn View_getState(this: &View) -> Object;
    #[wasm_bindgen(method, js_class = "View", js_name = getEphemeralState)]
    pub fn View_getEphemeralState(this: &View) -> Object;
    #[wasm_bindgen(method, js_class = "View", js_name = getIcon)]
    pub fn View_getIcon(this: &View) -> String;
    #[wasm_bindgen(method, js_class = "View", js_name = onHeaderMenu)]
    pub fn View_onHeaderMenu(this: &View, menu: &Menu);

    #[wasm_bindgen(method, js_class = "Workspace", js_name = requestSaveLayout)]
    pub fn Workspace_requestSaveLayout(this: &Workspace);
    #[wasm_bindgen(method, js_class = "Workspace", js_name = requestSaveHistory)]
    pub fn Workspace_requestSaveHistory(this: &Workspace);
    #[wasm_bindgen(method, js_class = "Workspace", js_name = changeLayout)]
    pub fn Workspace_changeLayout(this: &Workspace, workspace: &Object) -> Promise;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = getLayout)]
    pub fn Workspace_getLayout(this: &Workspace) -> Object;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = createLeafInParent)]
    pub fn Workspace_createLeafInParent(this: &Workspace, index: u32) -> WorkspaceLeaf;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = splitLeaf)]
    pub fn Workspace_splitLeaf(this: &Workspace, source: &WorkspaceItem, newLeaf: &WorkspaceItem, direction: &SplitDirection, before: bool);
    #[wasm_bindgen(method, js_class = "Workspace", js_name = createLeafBySplit)]
    pub fn Workspace_createLeafBySplit(this: &Workspace, direction: &SplitDirection, before: bool) -> WorkspaceLeaf;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = splitActiveLeaf)]
    pub fn Workspace_splitActiveLeaf(this: &Workspace, direction: &SplitDirection, before: bool);
    #[wasm_bindgen(method, js_class = "Workspace", js_name = splitLeafOrActive)]
    pub fn Workspace_splitLeafOrActive(this: &Workspace, leaf: &WorkspaceLeaf, direction: SplitDirection) -> Promise;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = duplicateLeaf)]
    pub fn Workspace_duplicateLeaf(this: &Workspace, leaf: &WorkspaceLeaf, direction: &SplitDirection) -> WorkspaceLeaf;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = getUnpinnedLeaf)]
    pub fn Workspace_getUnpinnedLeaf(this: &Workspace, typ: &str) -> WorkspaceLeaf;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = getLeaf)]
    pub fn Workspace_getLeaf(this: &Workspace, newLeaf: bool) -> WorkspaceLeaf;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = openLinkText)]
    pub fn Workspace_openLinkText(this: &Workspace, sourcePath: &str, newLeaf: bool, openViewState: &OpenViewState) -> Promise;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = setActiveLeaf)]
    pub fn Workspace_setActiveLeaf(this: &Workspace, leaf: &WorkspaceLeaf, pushHistory: bool);
    #[wasm_bindgen(method, js_class = "Workspace", js_name = getLeafById)]
    pub fn Workspace_getLeafById(this: &Workspace, id: &str) -> WorkspaceLeaf;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = getGroupLeaves)]
    pub fn Workspace_getGroupLeaves(this: &Workspace, group: &str) -> Array;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = getMostRecentLeaf)]
    pub fn Workspace_getMostRecentLeaf(this: &Workspace) -> WorkspaceLeaf;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = getLeftLeaf)]
    pub fn Workspace_getLeftLeaf(this: &Workspace, shouldSplit: bool) -> WorkspaceLeaf;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = getRightLeaf)]
    pub fn Workspace_getRightLeaf(this: &Workspace, shouldSplit: bool) -> WorkspaceLeaf;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = getActiveViewOfType)]
    pub fn Workspace_getActiveViewOfType(this: &Workspace, typ: Function) -> JsValue;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = getActiveFile)]
    pub fn Workspace_getActiveFile(this: &Workspace) -> JsValue;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = iterateRootLeaves)]
    pub fn Workspace_iterateRootLeaves(this: &Workspace, callback: Function);
    #[wasm_bindgen(method, js_class = "Workspace", js_name = iterateAllLeaves)]
    pub fn Workspace_iterateAllLeaves(this: &Workspace, callback: Function);
    #[wasm_bindgen(method, js_class = "Workspace", js_name = getLeavesOfType)]
    pub fn Workspace_getLeavesOfType(this: &Workspace, viewType: &str);
    #[wasm_bindgen(method, js_class = "Workspace", js_name = detachLeavesOfType)]
    pub fn Workspace_detachLeavesOfType(this: &Workspace, viewType: &str);
    #[wasm_bindgen(method, js_class = "Workspace", js_name = revealLeaf)]
    pub fn Workspace_revealLeaf(this: &Workspace, leaf: &WorkspaceLeaf);
    #[wasm_bindgen(method, js_class = "Workspace", js_name = getLastOpenFiles)]
    pub fn Workspace_getLastOpenFiles(this: &Workspace) -> Array;
    #[wasm_bindgen(method, js_class = "Workspace", js_name = on)]
    pub fn Workspace_on(this: &Workspace, callback: Function, context: &Object);

}

