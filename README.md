# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
<ol>
    <li>
    Dalam pola Observer tradisional (seperti di buku Head First Design Patterns), penggunaan interface (atau Trait di Rust) sangat disarankan karena Publisher biasanya perlu memberi tahu berbagai class atau tipe Observer yang berbeda-beda. Interface memastikan bahwa apa pun class objeknya, mereka semua memiliki metode update() yang sama. <br> <br>
    Namun, dalam kasus khusus BambangShop ini, satu struct Model saja sudah cukup, karena implementasi kita bergantung pada webhook (HTTP request). Dari sudut pandang Publisher, semua subscriber diperlakukan persis sama: mereka hanyalah sebuah url tujuan yang siap menerima data (payload) JSON. Publisher tidak peduli apakah penerimanya adalah aplikasi mobile, web server, atau bot Discord. Kecuali kita berencana menambahkan observer internal (seperti observer khusus untuk logging internal atau pengirim email) di samping webhook, sebuah struct sederhana yang menyimpan URL sudah sangat memadai.
    </li>
    <br>
    <li>
    Meskipun secara teknis kita bisa saja menggunakan Vec (list), menggunakan DashMap (map/dictionary) sangat diperlukan dan jauh lebih unggul untuk kasus ini. Nilai id pada Program dan url pada Subscriber ditujukan agar bersifat unik. Jika kita menggunakan Vec, setiap kali ada user yang melakukan subscribe, kita harus mengecek seluruh isi list satu per satu (kompleksitas waktu O(n)) hanya untuk memastikan URL tersebut belum ada, demi mencegah duplikasi. Begitu juga saat unsubscribe, kita harus mencari lagi di seluruh list. Tetapi dengan DashMap, kita tidak perlu memastikan keunikan lagi karena DashMap sudah menjamin bahwa setiap key (dalam hal ini url) hanya bisa ada satu. Kita bisa langsung menambahkan atau menghapus subscriber berdasarkan URL tanpa perlu iterasi, sehingga kompleksitas waktu menjadi O(1). Selain itu, DashMap juga memberikan thread safety yang lebih baik jika kita berencana untuk mengakses data secara bersamaan dari beberapa thread.
    </li>
    <br>
    <li>
        <ul>
        <li>Singleton adalah pola desain creational yang memastikan sebuah class hanya memiliki satu instance (objek) global dan menyediakan satu titik akses global ke objek tersebut. </li>
        <br>
        <li>DashMap adalah struktur data konkuren yang dirancang khusus untuk membaca dan menulis data secara aman dari berbagai thread (thread-safe).</li>
        </ul> 
        <br>
    Di bahasa Rust, framework web seperti Rocket menggunakan banyak worker thread untuk menangani request HTTP secara bersamaan. Jika kita menerapkan pola Singleton yang di dalamnya menyimpan HashMap biasa, compiler Rust yang sangat ketat akan tetap memblokir kode kita. Hal ini karena beberapa thread bisa saja mencoba membaca dan menulis ke HashMap tersebut di waktu yang bersamaan, sehingga menyebabkan data race (tabrakan data).
    <br>
    <br>
    Untuk membuat sebuah Singleton menjadi thread-safe di Rust, kita harus membungkus datanya dengan alat sinkronisasi seperti Mutex atau RwLock (misalnya menjadi Mutex<HashMap>). DashMap pada dasarnya adalah alternatif dari Mutex<HashMap> yang sudah sangat dioptimalkan untuk konkurensi (sistem lock-nya diurus di balik layar agar lebih cepat). Jadi, meskipun kita menerapkan pola Singleton, kita tetap wajib menggunakan struktur data yang thread-safe seperti DashMap (atau Mutex) untuk memenuhi aturan keamanan multithreading di Rust.
    </li>
</ol>

#### Reflection Publisher-2

<ol>
    <li>
    Berdasarkan prinsip desain perangkat lunak, terutama Single Responsibility Principle (SRP) dan Separation of Concerns, menggabungkan logika bisnis dan akses penyimpanan data ke dalam satu "Model" (seperti pada arsitektur MVC tradisional) akan membuat class tersebut menjadi sangat besar dan sulit dikelola (fat model). Dengan memecahnya menjadi tiga bagian:
    <br>
    <br>
    <ol>
        <li>Model: Hanya berfokus untuk merepresentasikan struktur data (cetakan objek).</li>
        <li>Repository: Bertanggung jawab khusus untuk operasi akses data (seperti Create, Read, Update, Delete ke database atau struktur data di memori).</li>
        <li>Service: Bertanggung jawab khusus untuk logika bisnis (business rules). Service bertindak sebagai konduktor yang memvalidasi data dan mengoordinasikan Repository mana yang harus dipanggil.</li>
    </ol>
    <br>
    Pemisahan ini membuat kode menjadi lebih modular, mudah diuji secara terisolasi (unit testing), dan lebih fleksibel.
    </li>
    <br>
    <li>
    Jika kita hanya menggunakan Model untuk melakukan semuanya, akan terjadi tingkat ketergantungan (tight coupling) yang sangat tinggi antar model. Model-model tersebut akan menjelma menjadi "God Object" (objek yang tahu dan melakukan terlalu banyak hal).
    <br>
    <br>
    Misalnya saat sebuah Program baru ditambahkan, model Program harus menulis kodenya sendiri untuk mencari siapa saja Subscriber-nya, lalu memicu pembuatan Notification, dan mengirimkannya. Model Program menjadi sangat terikat dengan detail implementasi Subscriber dan Notification. Kompleksitas kode akan meningkat drastis; kode menjadi sulit dibaca, sulit dilacak (di-debug), dan rentan rusak. Perubahan kecil pada fitur Notifikasi bisa saja mengharuskan kita membongkar ulang kode di dalam model Program.
    </li>
    <br>
    <li>
    Postman sangat membantu dalam menguji backend API yang sedang saya kembangkan. Alat ini memungkinkan saya untuk bertindak sebagai klien (layaknya browser atau aplikasi mobile) untuk mengirimkan HTTP Request (GET, POST, DELETE) dan melihat HTTP Response beserta data JSON-nya secara langsung, tanpa harus repot-repot membuat tampilan antarmuka (frontend/UI) terlebih dahulu.
    <br>
    <br>
    Beberapa fitur Postman yang menurut saya sangat menarik dan akan sangat membantu dalam Group Project atau pengerjaan perangkat lunak di masa depan antara lain:
    <br>
    <br>
    <ol>
        <li>
        Collections: Memungkinkan tim untuk menyimpan, mengelompokkan, dan membagikan daftar request API secara rapi, sehingga seluruh anggota tim (terutama frontend dan backend) memiliki acuan API yang sama.
        </li>
        <li>
        Environments & Variables: Fitur ini memudahkan kita untuk menyimpan URL dasar sebagai variabel. Kita bisa dengan mudah beralih dari pengujian server lokal (http://localhost:8000) ke server production yang sudah di-deploy hanya dengan mengganti Environment, tanpa perlu mengedit URL di setiap request satu per satu.
        </li>
        <li>
        Automated Testing: Postman menyediakan tab Tests di mana kita bisa menulis script untuk memvalidasi apakah status response (misalnya harus 200 OK) dan format data kembaliannya sudah sesuai ekspektasi secara otomatis.
        </li>
    </ol>
</ol>

#### Reflection Publisher-3

<ol>
    <li>
    Dalam tutorial BambangShop, kita menggunakan Push model. Hal ini terlihat pada bagian kode di mana aplikasi Main (Publisher) secara proaktif membuat dan mengirimkan data secara penuh (yakni payload Notifikasi) langsung ke alamat URL masing-masing Subscriber melalui HTTP POST request.
    </li>
    <br>
    <li>
    Jika kita menggunakan Pull model (di mana Publisher hanya sekadar memberi sinyal "Hei, ada produk baru!" tanpa mengirim detail datanya, dan Subscriber yang harus "menarik/meminta" data tersebut):
    <br>
    <br>
    Kelebihan (Advantages): Subscriber memiliki kontrol penuh kapan mereka siap menerima atau memproses data. Ini akan mencegah server Subscriber kelebihan beban (overload) jika sewaktu-waktu terjadi lonjakan notifikasi yang sangat masif. Subscriber juga bisa memilih untuk hanya menarik data yang benar-benar mereka butuhkan.
    <br>
    <br>
    Kekurangan (Disadvantages): Sistem menjadi lebih lambat (latency tinggi) dan tidak se- real-time Push model. Akan terjadi inefisiensi pada jaringan karena memerlukan dua kali komunikasi bolak-balik (Pemberitahuan dari Publisher -> Request minta data dari Subscriber -> Pengiriman data dari Publisher). Padahal, data notifikasi di tutorial ini (seperti tipe dan judul produk) ukurannya sangat kecil dan akan jauh lebih efisien jika langsung dikirim sekaligus.
    </li>
    <br>
    <li>
    Jika kita tidak menggunakan multi-threading (seperti thread::spawn yang ada pada kode NotificationService), maka proses pengiriman notifikasi akan berjalan secara sekuensial (berurutan) dan blocking.
    Artinya, aplikasi Publisher harus menunggu pengiriman HTTP Request ke Subscriber pertama selesai (menunggu response atau timeout), baru kemudian berlanjut mengirim ke Subscriber kedua, dan seterusnya.
    <br>
    <br>
    Dampaknya akan sangat fatal jika jumlah Subscriber sudah mencapai ribuan, atau jika ada salah satu server Subscriber yang sedang mati/lambat. Proses (thread) utama pada aplikasi Publisher akan tertahan (bottleneck). Pengguna yang memicu notifikasi tersebut (misalnya, admin yang sedang mengklik tombol "Create Product") harus melihat layar loading yang sangat lama hanya karena sistem di belakang layar sedang sibuk mengantre untuk menelepon Subscriber satu per satu. Dengan multi-threading, semua pengiriman itu dijalankan secara bersamaan di latar belakang (background), sehingga pengguna tetap bisa berinteraksi dengan aplikasi tanpa gangguan.
    </li>
</ol>
