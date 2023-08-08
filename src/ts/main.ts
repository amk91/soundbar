import '../scss/styles.scss'
import { invoke } from "@tauri-apps/api/tauri";

type SoundbiteInfo = {
    name: string,
    volume: number,
    speed: number,
    keycode: number,
}

let selectedSoundbite: HTMLElement | null;

let soundbiteInfo: HTMLElement | null;

let soundbiteName: HTMLElement | null;
let soundbiteNameInput: HTMLInputElement | null;
let soundbiteVolumeRange: HTMLInputElement | null;
let soundbiteVolumeValue: HTMLLabelElement | null;
let soundbiteSpeedRange: HTMLInputElement | null;
let soundbiteSpeedValue: HTMLLabelElement | null;
let soundbiteKeycode: HTMLElement | null;

let isKeyRecording = false;
let sysKeyRecorded: string | null;
let keyRecorded: number | null;
let keyNameRecorded: string | null;

function setMaxHeight() {
    let soundbitesList = document.getElementById("soundbites-list");
    soundbitesList!.style.maxHeight = (window.innerHeight * 80 / 100) + "px";
}

function updateSoundbiteInfo(event: MouseEvent) {
    let element = event.target as HTMLElement;
    if (element.tagName === 'A')
    {
        selectedSoundbite?.classList.remove('active');
        selectedSoundbite = element;
        selectedSoundbite!.classList.add('active');

        invoke('get_soundbite', { name: selectedSoundbite.textContent })
            .then((response) => {
                soundbiteInfo?.removeAttribute('hidden');

                let info = response as SoundbiteInfo;
                soundbiteName!.textContent = info.name;
                soundbiteVolumeRange!.value = info.volume.toString();
                soundbiteVolumeValue!.textContent = info.volume.toString() + '%';
                soundbiteSpeedRange!.value = info.speed.toString();
                soundbiteSpeedValue!.textContent = info.speed.toString() + '%';
                if (info.keycode === 0) {
                    soundbiteKeycode!.textContent = 'N/D'
                } else {
                    soundbiteKeycode!.textContent = info.keycode.toString();
                }
            });
    }
    else
    {
        selectedSoundbite?.classList.remove('active');
        selectedSoundbite = null;
        soundbiteName!.textContent = '';
        soundbiteInfo?.setAttribute('hidden', '');
    }
}

// pub enum SysKeyCode {
//     SHIFT = 0x10,
//     CTRL = 0x11,
//     ALT = 0x12,
//     LSHIFT = 0xA0,
//     RSHIFT = 0xA1,
//     LCTRL = 0xA2,
//     RCTRL = 0xA3,
//     LALT = 0xA4,
//     RALT = 0xA5,
// }

function startRecording() {
    isKeyRecording = true;
    
    let button = document.querySelector('#record-key') as HTMLButtonElement;
    button!.style.background = '#e14b4b';

    document.addEventListener('keydown', recordKey);
}

function stopRecording(save: boolean) {
    isKeyRecording = false;

    let button = document.querySelector('#record-key') as HTMLButtonElement;
    button!.style.background = '';

    document.removeEventListener('keydown', recordKey);

    if (save && keyRecorded != null) {
        let keycode = 0;
        let sysKeycode = 0;
        if (sysKeyRecorded != null) {
            switch (sysKeyRecorded) {
                case 'AltLeft':
                    sysKeycode = 0xA4;
                    break;
                case 'AltRight':
                    sysKeycode = 0xA5;
                    break;
                case 'CtrlLeft':
                    sysKeycode = 0xA2;
                    break;
                case 'CtrlRight':
                    sysKeycode = 0xA3;
                    break;
                case 'ShiftLeft':
                    sysKeycode = 0xA0;
                    break;
                case 'ShiftRight':
                    sysKeycode = 0xA1;
                    break;
            }

            if (sysKeycode != 0) {
                keycode = sysKeycode << 2;
            }
        }

        keycode |= keyRecorded

        invoke('set_keytask_code', {
            name: selectedSoundbite!.textContent,
            keytaskCode: keycode
        }).then((_) => {
            let keyCombinatinString = '';
            if (sysKeyRecorded && sysKeyRecorded.length > 0) {
                keyCombinatinString = sysKeyRecorded + ' + ';
            }

            keyCombinatinString += keyNameRecorded;
            soundbiteKeycode!.textContent = keyCombinatinString;
        }).catch((err) => {
            console.log(err);
        });

        keyRecorded = null;
    }
}

function recordKey(event: KeyboardEvent) {
    if (event.code === 'Escape') {
        stopRecording(false);
    } else if (event.code === 'Enter') {
        stopRecording(true);
    } else if (event.key === 'Alt' || event.key === 'Control' || event.key === 'Shift') {
        sysKeyRecorded = event.key;
    } else {
        keyRecorded = event.keyCode;
        keyNameRecorded = event.key;
    }
}

function playSound(_: MouseEvent) {
    invoke('play_soundbite', { name: selectedSoundbite!.textContent })
}

function stopSound(_: MouseEvent) {
    invoke('stop_soundbite', { name: selectedSoundbite!.textContent })
}

function assignKeyToSoundbite(_: MouseEvent) {
    if (!isKeyRecording) {
        startRecording();
    } else {
        stopRecording(true);
    }
}

function removeKeyFromSoundbite(_: MouseEvent) {
    console.log("removeKeyFromSoundbite fired")
}

function addSoundbiteToList(name: string) {
    let newItem = document.createElement('a');
    newItem.classList.add('list-group-item');
    newItem.classList.add('list-group-item-action');
    newItem.textContent = name;

    let soundbitesList = document.getElementById('soundbites-list');
    soundbitesList?.appendChild(newItem);
}

function onAddSoundbite(event: Event) {
    const target = event.target as HTMLInputElement;
    let file = target?.files![0];
    if (file) {
        let reader = new FileReader();
        reader.readAsArrayBuffer(file);
        reader.onloadend = (event) => {
            if (event.target?.readyState == FileReader.DONE) {
                var arrayBuffer = event.target.result as ArrayBuffer,
                    array = new Uint8Array(arrayBuffer);

                var buffer = []
                for (var i = 0; i < array.length; ++i) {
                    buffer.push(array[i])
                }

                let filename = file.name.replace(/\.[^/.]+$/, "")

                invoke("add_soundbite", { buffer: buffer, name: filename})
                    .then((message) => {
                        addSoundbiteToList(message as string);
                    })
                    .catch((error) => console.log(error));
            }
        };
    }
}

function updateSoundbiteName() {
    if (soundbiteNameInput!.value.length > 0) {
        invoke('set_name', {
            name: soundbiteName!.textContent,
            newName: soundbiteNameInput!.value
        }).then((_) => {
            selectedSoundbite!.textContent
            = soundbiteName!.textContent
            = soundbiteNameInput!.value;
        })
    }
}

function onRemoveSoundbite(_: MouseEvent) {
    invoke('remove_soundbite', { name: selectedSoundbite?.textContent })
    .then((_) => {
        selectedSoundbite?.remove();
        soundbiteInfo?.setAttribute('hidden', '');
        soundbiteName!.textContent = '';
    })
}

function onSettings(_: MouseEvent) {
    //TODO: display submenu for various settings
    console.log("onSettings fired");
}

window.addEventListener("DOMContentLoaded", () => {
    soundbiteInfo = document.getElementById("soundbite-info");
    soundbiteName = document.getElementById('soundbite-name');
    soundbiteNameInput = document.getElementById('soundbite-name-input') as HTMLInputElement;
    soundbiteVolumeRange = document.getElementById('volume-range') as HTMLInputElement;
    soundbiteVolumeValue = document.getElementById('volume-range-value') as HTMLLabelElement;
    soundbiteSpeedRange = document.getElementById('speed-range') as HTMLInputElement;
    soundbiteSpeedValue = document.getElementById('speed-range-value') as HTMLLabelElement;
    soundbiteKeycode = document.getElementById('keycode-value');

    soundbiteName!.onclick = (_) => {
        soundbiteName!.hidden = true;

        soundbiteNameInput!.value = soundbiteName!.textContent as string;

        soundbiteNameInput!.hidden = false;
        soundbiteNameInput!.focus();
    }

    soundbiteNameInput!.addEventListener('focusout', (_) => {
        updateSoundbiteName();

        soundbiteName!.hidden = false;
        soundbiteNameInput!.hidden = true;
    })

    soundbiteNameInput!.onkeydown = (event) => {
        if (event.key === 'Enter') {
            soundbiteName!.hidden = false;
            soundbiteNameInput!.hidden = true;
        } else if (event.key === 'Escape') {
            soundbiteNameInput!.value = '';

            soundbiteName!.hidden = false;
            soundbiteNameInput!.hidden = true;
        }
    }

    setMaxHeight();
    window.onresize = (_) => setMaxHeight();

    let volumeRange = document.querySelector<HTMLInputElement>('#volume-range');
    volumeRange!.onchange = (_) => {
        let volumeRangeValue = document.getElementById('volume-range-value');
        volumeRangeValue!.textContent = volumeRange!.value + "%";
    };
    volumeRange!.onmousemove = (_) => {
        let volumeRangeValue = document.getElementById('volume-range-value');
        volumeRangeValue!.textContent = volumeRange!.value + "%";
    };

    let speedRange = document.querySelector<HTMLInputElement>('#speed-range');
    speedRange!.onchange = (_) => {
        let speedRangeValue = document.getElementById('speed-range-value');
        speedRangeValue!.textContent = speedRange!.value + "%";
    };
    speedRange!.onmousemove = (_) => {
        let speedRangeValue = document.getElementById('speed-range-value');
        speedRangeValue!.textContent = speedRange!.value + "%";
    };

    document.getElementById('play-sound')!.onclick = playSound;
    document.getElementById('stop-sound')!.onclick = stopSound;
    document.getElementById('record-key')!.onclick = assignKeyToSoundbite;
    document.getElementById('remove-key')!.onclick = removeKeyFromSoundbite;

    let addSoundbiteInput = document.getElementById('add-soundbite-input');
    addSoundbiteInput!.oninput = (event) => {
        onAddSoundbite(event);
    }

    let addSoundbiteButton = document.getElementById('add-soundbite');
    addSoundbiteButton!.onclick = (_) => {
        addSoundbiteInput!.click();
    }

    document.getElementById('remove-soundbite')!.onclick = onRemoveSoundbite;
    document.getElementById('settings')!.onclick = onSettings;
    document.getElementById("soundbites-list")!.onclick = updateSoundbiteInfo;

    invoke("get_soundbites")
        .then((response) => {
            let soundbites = response as [string];
            for (let soundbite of soundbites) {
                addSoundbiteToList(soundbite);
            }
        })
});
