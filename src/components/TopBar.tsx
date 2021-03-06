import { appWindow } from "@tauri-apps/api/window";
import { useStore } from "../hooks/store";
import { useFs } from "../hooks/useFs";
import { FC, useState } from "react";
import { AiOutlineMenu } from "react-icons/ai";
import { FiSidebar } from "react-icons/fi";
import {
  VscChromeClose,
  VscChromeMaximize,
  VscChromeMinimize,
  VscFile,
  VscFolderOpened,
  VscInfo,
  VscChromeRestore,
} from "react-icons/vsc";
import { WiMoonFull } from "react-icons/wi";
import { styled } from "../theme";
import { Content, Item, ItemIcon, Menu, Trigger } from "./ui/Menu";

const TopBarContainer = styled("div", {
  fontFamily: "sans-serif",
  fontSize: 12,
  top: 0,
  left: 0,
  right: 0,
  height: 30,
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  userSelect: "none",
  zIndex: 100,
  paddingLeft: 10,
  paddingRight: 10,
  boxSizing: "border-box",
  color: "#fff",
  borderBottom: "1px solid rgba(15, 15, 15, 0.43)",
  borderRadius: 3,
});

const TopIconLeft = styled("div", {
  marginRight: 10,
  cursor: "pointer",
  opacity: 0.5,
  "&:hover": {
    opacity: 1,
  },
});
const TopIconRight = styled(TopIconLeft, {
  marginLeft: 10,
  marginRight: 0,
});

export const TopBar: FC = () => {
  const showSide = useStore((s: any) => s.showSide);
  const set = useStore((s: any) => s.set);
  const { openFile, openDir } = useFs();
  let [maximized, setMaximized] = useState(false);

  const hadleWindowResize = () => {
    appWindow.toggleMaximize();
    setMaximized(!maximized);
  };

  let title = "uNote";

  return (
    <TopBarContainer className="titlebar" data-tauri-drag-region="">
      <div style={{ flex: 1, display: "flex" }} data-tauri-drag-region="">
        <Menu>
          <Trigger>
            <TopIconLeft as={AiOutlineMenu} />{" "}
          </Trigger>
          <Content sideOffset={10}>
            <Item onSelect={openFile}>
              <ItemIcon as={VscFile} />
              Open File
            </Item>
            <Item onSelect={openDir}>
              <ItemIcon as={VscFolderOpened} />
              Open Directory
            </Item>
          </Content>
        </Menu>

        <TopIconLeft
          as={FiSidebar}
          onClick={() => set({ showSide: !showSide })}
          style={{ opacity: showSide ? 1 : 0.3 }}
        />
      </div>
      <div>{title}</div>
      <div
        style={{ flex: 1, justifyContent: "flex-end", display: "flex" }}
        data-tauri-drag-region=""
      >
        <>
          <TopIconRight
            as={VscChromeMinimize}
            onClick={() => appWindow.minimize()}
          />
          <TopIconRight
            as={maximized ? VscChromeRestore : VscChromeMaximize}
            onClick={() => hadleWindowResize()}
          />
          <TopIconRight as={VscChromeClose} onClick={() => appWindow.close()} />
        </>
      </div>
    </TopBarContainer>
  );
};
