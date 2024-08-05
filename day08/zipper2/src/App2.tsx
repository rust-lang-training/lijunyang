import './style.css';
import { open } from '@tauri-apps/api/dialog';
import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
const items: Array<Entry> = [];
interface Entry {
    entryType: string;
    size: string;
    compressedSize: string;
    name: string;
}
async function openFile() {
    const filename = await open({
        filters: [
            {
                name: 'zip file',
                extensions: ['zip'],
            },
        ],
    });
    if (!filename) {
        return;
    }
    console.log(`file to open: ${filename}`);
    items.splice(0);
    const res = await invoke('list_entries', { fileName: filename });
}
let unListenFunctions: Array<Function> = [];

const App2 = () => {
    const [items, setItems] = useState<Entry[]>([]);
    useEffect(() => {
        (async () => {
            const unListenFunc = await listen(
                'list-entries-result',
                (event) => {
                    console.log(event, 'event');
                    const payload = event.payload as any;
                    console.log(payload, 'payload');

                    if (!payload.result) {
                        console.log(
                            `error while unzipping file: ${payload.message}`
                        );
                        return;
                    }
                    payload.entries.forEach((ent: any) => {
                        items.push(ent);
                    });
                    console.log(items, 'items');
                    setItems([...items]);
                }
            );

            unListenFunctions.push(unListenFunc);
        })();
        return () => {
            for (let unlistenFunc of unListenFunctions) {
                unlistenFunc();
            }
        };
    });
    return (
        <div>
            <button className='btn' onClick={openFile}>
                Open file
            </button>
            <table>
                <thead>
                    <tr className='bg-gray-200 p-10'>
                        <th className='w-300'>Name</th>
                        <th>Type</th>
                        <th>Compressed Size (Bytes)</th>
                        <th>Size (Bytes)</th>
                    </tr>
                </thead>
                <tbody>
                    {items.map((item, index) => {
                        return (
                            <tr key={index}>
                                <td className='w-300'>{item.name}</td>
                                <td>{item.entryType}</td>
                                <td>{item.compressedSize}</td>
                                <td>{item.size}</td>
                            </tr>
                        );
                    })}
                </tbody>
            </table>
        </div>
    );
};
export default App2;
