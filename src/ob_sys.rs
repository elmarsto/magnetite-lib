use wasm_bindgen::prelude::*;
use js_sys::{ Promise, Object, Array, ArrayBuffer, Function };
use web_sys::{ HtmlElement };

// low-level APIs (plumbing, not porcelain; intentionally brutalist; eventually I want this file autogenned)
// note we preserve camelCase here; contextual cue that you're dealing with import
// TODO: build thing to turn typescript.d.ts files into imports via AST

#[wasm_bindgen]
extern "C" {
    /* Classes */
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
    pub type WorkspaceItem;

    #[wasm_bindgen(extends = WorkspaceItem)]
    pub type WorkspaceLeaf;

    #[wasm_bindgen(extends = WorkspaceItem)]
    pub type WorkspaceParent;

    #[wasm_bindgen(extends = Object)]
    pub type WorkspaceRibbon;

    #[wasm_bindgen(extends = WorkspaceSplit)]
    pub type WorkspaceSidedock;

    #[wasm_bindgen(extends = WorkspaceParent)]
    pub type WorkspaceSplit;

    #[wasm_bindgen(extends = WorkspaceParent)]
    pub type WorkspaceTabs;

    /* Loose Functions */

    #[wasm_bindgen]
    pub fn addIcon(iconId: &str, svgContent: &str);

    #[wasm_bindgen]
    pub fn getAllTags(iconId: &str, svgContent: &str) -> Array;

    #[wasm_bindgen]
    pub fn getLinkPath(linktext: &str) -> String;

    #[wasm_bindgen]
    pub fn iterateCacheRefs(cache: &CachedMetadata, cb: &Function) -> bool;

    #[wasm_bindgen]
    pub fn iterateRefs(refs: &Array, cb: &Function) -> bool;

    #[wasm_bindgen]
    pub fn normalizePath(path: &str) -> String;

    #[wasm_bindgen]
    pub fn resolveSubpath(cache: &CachedMetadata, subpath: &str) -> SubpathResult;

    #[wasm_bindgen]
    pub fn setIcon(parent: &HtmlElement, iconId: &str, size: u32);

    /* Methods */

    #[wasm_bindgen(method, js_class = "AbstractTextComponent", js_name = getValue)]
    pub fn AbstractTextComponent_getValue(this: &AbstractTextComponent) -> String;

    #[wasm_bindgen(constructor, js_class = "AbstractTextComponent")]
    pub fn AbstractTextComponent_new() -> AbstractTextComponent;

    #[wasm_bindgen(method, js_class = "AbstractTextComponent", js_name = onChange)]
    pub fn AbstractTextComponent_onChange(this: &AbstractTextComponent, callback: Function);

    #[wasm_bindgen(method, js_class = "AbstractTextComponent", js_name = onChanged)]
    pub fn AbstractTextComponent_onChanged(this: &AbstractTextComponent);

    #[wasm_bindgen(method, js_class = "AbstractTextComponent", js_name = setPlaceholder)]
    pub fn AbstractTextComponent_setPlaceholder(this: &AbstractTextComponent, placeholder: &str);

    #[wasm_bindgen(method, js_class = "AbstractTextComponent", js_name = setValue)]
    pub fn AbstractTextComponent_setValue(this: &AbstractTextComponent, value: &str) -> AbstractTextComponent;


    #[wasm_bindgen(method, js_class = "App")]
    pub fn App_on(app: &App, eventName: &str, callback: Function, ctx: JsValue);


    #[wasm_bindgen(method, js_class = "BaseComponent", js_name = then)]
    pub fn BaseComponent_then(this: &BaseComponent, cb: &Function);

    #[wasm_bindgen(constructor, js_class = "ButtonComponent")]
    pub fn ButtonComponent_new(containerEl: &HtmlElement) -> ButtonComponent;

    #[wasm_bindgen(method, js_class = "ButtonComponent", js_name = onClick)]
    pub fn ButtonComponent_onClick(this: &ButtonComponent, cb: &Function);

    #[wasm_bindgen(method, js_class = "ButtonComponent", js_name = setButtonText)]
    pub fn ButtonComponent_setButtonText(this: &ButtonComponent,name: &str);

    #[wasm_bindgen(method, js_class = "ButtonComponent", js_name = setClass)]
    pub fn ButtonComponent_setClass(this: &ButtonComponent, cls: &str);

    #[wasm_bindgen(method, js_class = "ButtonComponent", js_name = setCta)]
    pub fn ButtonComponent_setCta(this: &ButtonComponent);

    #[wasm_bindgen(method, js_class = "ButtonComponent", js_name = setIcon)]
    pub fn ButtonComponent_setIcon(this: &ButtonComponent, icon: &str);


    #[wasm_bindgen(method, js_class = "Command", js_name = callback)]
    pub fn Command_callback(this: &Command) -> JsValue;

    #[wasm_bindgen(method, js_class = "Command", js_name = checkCallback)]
    pub fn Command_checkCallback(this: &Command, checking: bool) -> bool;


    #[wasm_bindgen(method, js_class = "Component", js_name = addChild)]
    pub fn Component_addChild(this: &Component, child: &Component);

    #[wasm_bindgen(method, js_class = "Component", js_name = load)]
    pub fn Component_load(this: &Component);

    #[wasm_bindgen(method, js_class = "Component", js_name = unload)]
    pub fn Component_unload(this: &Component);

    #[wasm_bindgen(method, js_class = "Component", js_name = register)]
    pub fn Component_register(this: &Component, cb: &Function);

    #[wasm_bindgen(method, js_class = "Component", js_name = removeChild)]
    pub fn Component_removeChild(this: &Component, child: &Component) -> bool;


    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = copy)]
    pub fn DataAdapter_copy(this: &DataAdapter, normalizedPath: &str, normalizedNewPath: &str);

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = exists)]
    pub fn DataAdapter_exists(this: &DataAdapter, normalizedPath: &str, sensitive: bool) -> Promise;

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = getName)]
    pub fn DataAdapter_getName(this: &DataAdapter) -> String;

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = getResourcePath)]
    pub fn DataAdapter_getResourcePath(this: &DataAdapter, normalizedPath: &str) -> Promise;

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = list)]
    pub fn DataAdapter_list(this: &DataAdapter, normalizedPath: &str) -> Promise;

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = mkdir)]
    pub fn DataAdapter_mkdir(this: &DataAdapter, normalizedPath: &str) -> Promise;

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = read)]
    pub fn DataAdapter_read(this: &DataAdapter, normalizedPath: &str);

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = readBinary)]
    pub fn DataAdapter_readBinary(this: &DataAdapter, normalizedPath: &str);

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = remove)]
    pub fn DataAdapter_remove(this: &DataAdapter, normalizedPath: &str) -> Promise;

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = rename)]
    pub fn DataAdapter_rename(this: &DataAdapter, normalizedPath: &str, normalizedNewPath: &str) -> Promise;

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = rmdir)]
    pub fn DataAdapter_rmdir(this: &DataAdapter, normalizedPath: &str, recursive: bool) -> Promise;

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = setCtime)]
    pub fn DataAdapter_setCtime(this: &DataAdapter, normalizedPath: &str, ctime: u32) -> Promise;

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = setCtime)]
    pub fn DataAdapter_setMtime(this: &DataAdapter, normalizedPath: &str, mtime: u32) -> Promise;

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = trashLocal)]
    pub fn DataAdapter_trashLocal(this: &DataAdapter, normalizedPath: &str);

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = trashSystem)]
    pub fn DataAdapter_trashSystem(this: &DataAdapter,normalizedPath: &str);

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = write)]
    pub fn DataAdapter_write(this: &DataAdapter, normalizedPath: &str, data: &str, immediate: &Function) -> Promise;

    #[wasm_bindgen(method, js_class = "DataAdapter", js_name = writeBinary)]
    pub fn DataAdapter_writeBinary(this: &DataAdapter, normalizedPath: &str, data: &ArrayBuffer, immediate: &Function) -> Promise;


    #[wasm_bindgen(method, js_class = "DropdownComponent", js_name = addOption)]
    pub fn DropdownComponent_addOption(this: &DropdownComponent, value: &str, display: &str);

    #[wasm_bindgen(method, js_class = "DropdownComponent", js_name = addOptions)]
    pub fn DropdownComponent_addOptions(this: &DropdownComponent, options: &Object);

    #[wasm_bindgen(method, js_class = "DropdownComponent", js_name = getValue)]
    pub fn DropdownComponent_getValue(this: &DropdownComponent) -> String;

    #[wasm_bindgen(method, js_class = "DropdownComponent", js_name = onChange)]
    pub fn DropdownComponent_onChange(this: &DropdownComponent, cb: &Function);

    #[wasm_bindgen(method, js_class = "DropdownComponent", js_name = setValue)]
    pub fn DropdownComponent_setValue(this: &DropdownComponent, value: &str);


    #[wasm_bindgen(method, js_class = "Events", js_name = off)]
    pub fn Events_off(name: &str, cb: &Function);

    #[wasm_bindgen(method, js_class = "Events", js_name = offRef)]
    pub fn Events_offRef(this: &Events, raef: &EventRef);

    #[wasm_bindgen(method, js_class = "Events", js_name = on)]
    pub fn Events_on(name: &str, cb: &Function, ctx: &Object) -> EventRef;

    #[wasm_bindgen(method, js_class = "Events", js_name = trigger)]
    pub fn Events_trigger(this: &Events, name: &str, data: Array);

    #[wasm_bindgen(method, js_class = "Events", js_name = tryTrigger)]
    pub fn Events_tryTrigger(this: &Events, evt: &EventRef, args: Array);


    #[wasm_bindgen(constructor, js_class = "ExtraButtonComponent")]
    pub fn ExtraButtonComponent_new() -> ExtraButtonComponent;

    #[wasm_bindgen(method, js_class = "ExtraButtonComponent", js_name = onClick)]
    pub fn ExtraButtonComponent_onClick(this: &ExtraButtonComponent, cb: &Function);

    #[wasm_bindgen(method, js_class = "ExtraButtonComponent", js_name = setTooltip)]
    pub fn ExtraButtonComponent_setTooltip(this: &ExtraButtonComponent, tooltip: &str);

    #[wasm_bindgen(method, js_class = "ExtraButtonComponent", js_name = setIcon)]
    pub fn ExtraButtonComponent_setIcon(icon: &str);


    #[wasm_bindgen(js_namespace = FileSystemDataAdapter)]
    pub fn readLocalFile(this: &ExtraButtonComponent, path: &str);

    #[wasm_bindgen(js_namespace = FileSystemDataAdapter)]
    pub fn mkdir(this: &ExtraButtonComponent, path: &str);

    #[wasm_bindgen(method, js_class = "FileSystemAdapter", js_name = getFullPath)]
    pub fn FileSystemAdapter_getFullPath(this: &FileSystemAdapter, normalizedPath: &str) -> String;

    #[wasm_bindgen(method, js_class = "FileView", js_name = canAcceptExtension)]
    pub fn FileView_canAcceptExtension(this: &FileView, extension: &str);

    #[wasm_bindgen(method, js_class = "FileView", js_name = getDisplayText)]
    pub fn FileView_getDisplayText(this: &FileView) -> String;

    #[wasm_bindgen(method, js_class = "FileView", js_name = load)]
    pub fn FileView_load(this: &FileView);

    #[wasm_bindgen(constructor, js_class = "FileView")]
    pub fn FileView_new() -> FileView;

    #[wasm_bindgen(method, js_class = "FileView", js_name = onHeaderMenu)]
    pub fn FileView_onHeaderMenu(this: &FileView, menu: &Menu);

    #[wasm_bindgen(method, js_class = "FileView", js_name = onMoreOptionsMenu)]
    pub fn FileView_onMoreOptionsMenu(this: &FileView, menu: &Menu);


    #[wasm_bindgen(constructor, js_class = "HoverPopover")]
    pub fn HoverPopover_new(parent: &HoverParent, targetEl: &HtmlElement) -> HoverPopover;


    #[wasm_bindgen(method, js_class = "ItemView", js_name = addAction)]
    pub fn ItemView_addAction(this: &ItemView, icon: &str, title: &str, cb: &Function, size: u32) -> HtmlElement;

    #[wasm_bindgen(constructor, js_class = "ItemView")]
    pub fn ItemView_new(leaf: &WorkspaceLeaf) -> ItemView;

    #[wasm_bindgen(method, js_class = "ItemView", js_name = onHeaderMenu)]
    pub fn ItemView_onHeaderMenu(this: &ItemView, menu: &Menu);

    #[wasm_bindgen(method, js_class = "ItemView", js_name = onMoreOptionsMenu)]
    pub fn ItemView_onMoreOptionsMenu(this: &ItemView, menu: &Menu);


    #[wasm_bindgen(method, js_class = "Plugin", js_name = addCommand)]
    pub fn Plugin_addCommand(this: &Plugin, command: &Command);

    #[wasm_bindgen(method, js_class = "Plugin", js_name = addRibbonIcon)]
    pub fn Plugin_addRibbonIcon(this: &Plugin, icon: &str, title: &str) -> HtmlElement;

    #[wasm_bindgen(method, js_class = "Plugin", js_name = addSettingTab)]
    pub fn Plugin_addSettingTab(this: &Plugin, settingTab: &PluginSettingTab);

    #[wasm_bindgen(method, js_class = "Plugin", js_name = addStatusBarItem)]
    pub fn Plugin_addStatusBarItem(this: &Plugin) -> HtmlElement;

    #[wasm_bindgen(method, js_class = "Plugin", js_name = loadData)]
    pub fn Plugin_loadData(this: &Plugin) -> Promise;

    #[wasm_bindgen(method, js_class = "Plugin", js_name = registerExtensions)]
    pub fn Plugin_registerExtensions(this: &Plugin, extensions: &Array, viewType: &str);

    #[wasm_bindgen(method, js_class = "Plugin", js_name = registerView)]
    pub fn Plugin_registerView(this: &Plugin, viewCreator: &ViewCreator);

    #[wasm_bindgen(method, js_class = "Plugin", js_name = saveData)]
    pub fn Plugin_saveData(this: &Plugin, data: &Object) -> Promise;


    #[wasm_bindgen(method, js_class = "Vault", js_name = delete)]
    pub fn Vault_delete(this: &Vault) -> Promise;

    #[wasm_bindgen(method, js_class = "Vault", js_name = cachedRead)]
    pub fn Vault_cachedRead(this: &Vault, file: &TFile) -> Promise;

    #[wasm_bindgen(method, js_class = "Vault", js_name = copy)]
    pub fn Vault_copy(this: &Vault, file: &TFile, newPath: &str) -> Promise;

    #[wasm_bindgen(method, js_class = "Vault", js_name = create)]
    pub fn Vault_create(this: &Vault, path: &str, data: &str) -> Promise;

    #[wasm_bindgen(method, js_class = "Vault", js_name = createBinary)]
    pub fn Vault_createBinary(this: &Vault, path: &str, data: &ArrayBuffer) -> Promise;

    #[wasm_bindgen(method, js_class = "Vault", js_name = createFolder)]
    pub fn Vault_createFolder(this: &Vault, path: &str) -> Promise;

    #[wasm_bindgen(method, js_class = "Vault", js_name = getAbstractFileByPath)]
    pub fn Vault_getAbstractFileByPath(this: &Vault, path: &str) -> TAbstractFile;

    #[wasm_bindgen(method, js_class = "Vault", js_name = getAllLoadedFiles)]
    pub fn Vault_getAllLoadedFiles(this: &Vault) -> Array;

    #[wasm_bindgen(method, js_class = "Vault", js_name = getName)]
    pub fn Vault_getName(this: &Plugin) -> String;

    #[wasm_bindgen(method, js_class = "Vault", js_name = getResourcePath)]
    pub fn Vault_getResourcePath(this: &Vault, file: &TFile) -> Promise;

    #[wasm_bindgen(method, js_class = "Vault", js_name = getRoot)]
    pub fn Vault_getRoot(this: &Vault) -> TFolder;

    #[wasm_bindgen(method, js_class = "Vault", js_name = modify)]
    pub fn Vault_modify(this: &Vault, file: &TFile, data: &str);

    #[wasm_bindgen(method, js_class = "Vault", js_name = modifyBinary)]
    pub fn Vault_modifyBinary(this: &Vault, file: &TFile, data: &ArrayBuffer);

    #[wasm_bindgen(method, js_class = "Vault", js_name = on)]
    pub fn Vault_on(this: &Vault, name: &str, callback: &Function, context: JsValue) -> EventRef;

    #[wasm_bindgen(method, js_class = "Vault", js_name = read)]
    pub fn Vault_read(this: &Vault, file: &TFile) -> Promise;

    #[wasm_bindgen(method, js_class = "Vault", js_name = readBinary)]
    pub fn Vault_readBinary(this: &Vault, file: &ArrayBuffer) -> Promise;

    #[wasm_bindgen(method, js_class = "Vault", js_name = recurseChildren)]
    pub fn Vault_recurseChildren(this: &Vault, root: &TFolder, cb: &Function);

    #[wasm_bindgen(method, js_class = "Vault", js_name = rename)]
    pub fn Vault_rename(this: &Vault, file: &TFile, newPath: &str) -> Promise;

    #[wasm_bindgen(method, js_class = "Vault", js_name = trash)]
    pub fn Vault_trash(this: &Vault, file: &TAbstractFile, system: bool) -> Promise;


    #[wasm_bindgen(method, js_class = "View", js_name = getEphemeralState)]
    pub fn View_getEphemeralState(this: &View) -> Object;

    #[wasm_bindgen(method, js_class = "View", js_name = getIcon)]
    pub fn View_getIcon(this: &View) -> String;

    #[wasm_bindgen(method, js_class = "View", js_name = getState)]
    pub fn View_getState(this: &View) -> Object;

    #[wasm_bindgen(method, js_class = "View", js_name = getViewType)]
    pub fn View_getViewType(this: &View) -> String;

    #[wasm_bindgen(constructor, js_class = "View")]
    pub fn View_new(leaf: &WorkspaceLeaf) -> View;

    #[wasm_bindgen(method, js_class = "View", js_name = onClose)]
    pub fn View_onClose(this: &View) -> Promise;

    #[wasm_bindgen(method, js_class = "View", js_name = onHeaderMenu)]
    pub fn View_onHeaderMenu(this: &View, menu: &Menu);

    #[wasm_bindgen(method, js_class = "View", js_name = onOpen)]
    pub fn View_onOpen(this: &View) -> Promise;


    #[wasm_bindgen(method, js_class = "Workspace", js_name = changeLayout)]
    pub fn Workspace_changeLayout(this: &Workspace, workspace: &Object) -> Promise;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = createLeafBySplit)]
    pub fn Workspace_createLeafBySplit(this: &Workspace, direction: &SplitDirection, before: bool) -> WorkspaceLeaf;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = createLeafInParent)]
    pub fn Workspace_createLeafInParent(this: &Workspace, index: u32) -> WorkspaceLeaf;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = detachLeavesOfType)]
    pub fn Workspace_detachLeavesOfType(this: &Workspace, viewType: &str);

    #[wasm_bindgen(method, js_class = "Workspace", js_name = duplicateLeaf)]
    pub fn Workspace_duplicateLeaf(this: &Workspace, leaf: &WorkspaceLeaf, direction: &SplitDirection) -> WorkspaceLeaf;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = getActiveViewOfType)]
    pub fn Workspace_getActiveViewOfType(this: &Workspace, typ: Function) -> JsValue;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = getActiveFile)]
    pub fn Workspace_getActiveFile(this: &Workspace) -> JsValue;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = getGroupLeaves)]
    pub fn Workspace_getGroupLeaves(this: &Workspace, group: &str) -> Array;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = getLayout)]
    pub fn Workspace_getLastOpenFiles(this: &Workspace) -> Array;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = getLeaf)]
    pub fn Workspace_getLayout(this: &Workspace) -> Object;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = getLastOpenFiles)]
    pub fn Workspace_getLeaf(this: &Workspace, newLeaf: bool) -> WorkspaceLeaf;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = getLeafById)]
    pub fn Workspace_getLeafById(this: &Workspace, id: &str) -> WorkspaceLeaf;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = getLeftLeaf)]
    pub fn Workspace_getLeftLeaf(this: &Workspace, shouldSplit: bool) -> WorkspaceLeaf;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = getLeavesOfType)]
    pub fn Workspace_getLeavesOfType(this: &Workspace, viewType: &str);

    #[wasm_bindgen(method, js_class = "Workspace", js_name = getMostRecentLeaf)]
    pub fn Workspace_getMostRecentLeaf(this: &Workspace) -> WorkspaceLeaf;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = getRightLeaf)]
    pub fn Workspace_getRightLeaf(this: &Workspace, shouldSplit: bool) -> WorkspaceLeaf;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = getUnpinnedLeaf)]
    pub fn Workspace_getUnpinnedLeaf(this: &Workspace, typ: &str) -> WorkspaceLeaf;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = on)]
    pub fn Workspace_on(this: &Workspace, callback: Function, context: &Object);

    #[wasm_bindgen(method, js_class = "Workspace", js_name = iterateAllLeaves)]
    pub fn Workspace_iterateAllLeaves(this: &Workspace, callback: Function);

    #[wasm_bindgen(method, js_class = "Workspace", js_name = iterateRootLeaves)]
    pub fn Workspace_iterateRootLeaves(this: &Workspace, callback: Function);

    #[wasm_bindgen(method, js_class = "Workspace", js_name = openLinkText)]
    pub fn Workspace_openLinkText(this: &Workspace, sourcePath: &str, newLeaf: bool, openViewState: &OpenViewState) -> Promise;

    #[wasm_bindgen(method, js_class = "Workspace", js_name = revealLeaf)]
    pub fn Workspace_revealLeaf(this: &Workspace, leaf: &WorkspaceLeaf);

    #[wasm_bindgen(method, js_class = "Workspace", js_name = requestSaveLayout)]
    pub fn Workspace_requestSaveLayout(this: &Workspace);

    #[wasm_bindgen(method, js_class = "Workspace", js_name = requestSaveHistory)]
    pub fn Workspace_requestSaveHistory(this: &Workspace);

    #[wasm_bindgen(method, js_class = "Workspace", js_name = setActiveLeaf)]
    pub fn Workspace_setActiveLeaf(this: &Workspace, leaf: &WorkspaceLeaf, pushHistory: bool);

    #[wasm_bindgen(method, js_class = "Workspace", js_name = splitActiveLeaf)]
    pub fn Workspace_splitActiveLeaf(this: &Workspace, direction: &SplitDirection, before: bool);

    #[wasm_bindgen(method, js_class = "Workspace", js_name = splitLeaf)]
    pub fn Workspace_splitLeaf(this: &Workspace, source: &WorkspaceItem, newLeaf: &WorkspaceItem, direction: &SplitDirection, before: bool);

    #[wasm_bindgen(method, js_class = "Workspace", js_name = splitLeafOrActive)]
    pub fn Workspace_splitLeafOrActive(this: &Workspace, leaf: &WorkspaceLeaf, direction: SplitDirection) -> Promise;


    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = detach)]
    pub fn WorkspaceLeaf_detach(this: &WorkspaceLeaf);

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = on)]
    pub fn WorkspaceLeaf_on(this: &WorkspaceLeaf, name: &str, callback: &Function, ctx: &Object) -> EventRef;

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = onResize)]
    pub fn WorkspaceLeaf_onResize (this: &WorkspaceLeaf);

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = openFile)]
    pub fn WorkspaceLeaf_openFile(this: &WorkspaceLeaf, file: &TFile, openState: &OpenViewState) -> Promise;

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = open)]
    pub fn WorkspaceLeaf_open(this: &WorkspaceLeaf, view: &View) -> Promise;

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = getDisplayText)]
    pub fn WorkspaceLeaf_getDisplayText(this: &WorkspaceLeaf) -> String;

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = getEphemeralState)]
    pub fn WorkspaceLeaf_getEphemeralState(this: &WorkspaceLeaf);

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = getIcon)]
    pub fn WorkspaceLeaf_getIcon(this: &WorkspaceLeaf);

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = getViewState)]
    pub fn WorkspaceLeaf_getViewState(this: &WorkspaceLeaf);

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = setEphemeralState)]
    pub fn WorkspaceLeaf_setEphemeralState(this: &WorkspaceLeaf, state: &Object);

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = setGroupMember)]
    pub fn WorkspaceLeaf_setGroupMember(this: &WorkspaceLeaf, other: &WorkspaceLeaf);

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = setGroup)]
    pub fn WorkspaceLeaf_setGroup(this: &WorkspaceLeaf, group: &str);

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = setPinned)]
    pub fn WorkspaceLeaf_setPinned(this: &WorkspaceLeaf, pinned: bool);

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = setViewState)]
    pub fn WorkspaceLeaf_setViewState(this: &WorkspaceLeaf, viewState: &ViewState, eState: &Object) -> Promise;

    #[wasm_bindgen(method, js_class = "WorkspaceLeaf", js_name = togglePinned)]
    pub fn WorkspaceLeaf_togglePinned(this: &WorkspaceLeaf);
}

