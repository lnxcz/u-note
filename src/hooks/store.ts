import create from "zustand";
import { combine } from "zustand/middleware";
import { persist } from "zustand/middleware";

export const useStore = create(
  persist(
    combine(
      {
        currentFilePaths: [] as string[],
        currentProjectPath: "",
        currentDirectoryPath: "",

        showSide: false,
        showAddItem: false as "file" | "folder" | false,
        scrollMode: false,
      },
      (set) => ({
        set,
      }),
    ),
    {
      name: "store", // unique name
    },
  ),
);
