import { getVersion } from '@tauri-apps/api/app';
import { relaunch } from '@tauri-apps/plugin-process';
import { check, type Update } from '@tauri-apps/plugin-updater';
import { useEffect, useState } from 'react';

export const App = () => {
    const [update, setUpdate] = useState<Update | null>(null);
    const [version, setVersion] = useState<string | null>(null);

    useEffect(() => {
        (async () => {
            setUpdate(await check());
            setVersion(await getVersion());
        })();
    }, []);

    return (
        <div>
            {update ? (
                <>
                    <h1>アップデートが見つかりました。</h1>
                    <p>
                        {update.currentVersion} → {update.version}
                    </p>
                    <button
                        onClick={async () => {
                            await update.downloadAndInstall(({ event }) => {
                                console.log(event);
                            });
                            await relaunch();
                        }}
                    >
                        アップデート
                    </button>
                </>
            ) : (
                <>
                    <p>アップデートが見つかりません。</p>
                    <p>{version ?? '???'}</p>
                </>
            )}
        </div>
    );
};
