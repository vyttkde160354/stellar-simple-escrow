# stellar-simple-escrow

<img width="1416" height="839" alt="image" src="https://github.com/user-attachments/assets/1148f9be-aff6-4359-bbca-bb04cfc4322f" />


TÊN DỰ ÁN:
💼 Stellar Simple Escrow Contract

VẤN ĐỀ (1 câu):
Trong giao dịch giữa hai bên không tin tưởng nhau (freelancer, marketplace), cần một cơ chế trung gian để đảm bảo thanh toán công bằng.

GIẢI PHÁP (1 câu):
Smart contract này trên Stellar cho phép khóa giao dịch giữa payer và payee, chỉ giải ngân khi điều kiện được xác nhận hoặc hoàn tiền nếu giao dịch bị hủy.

TÍNH NĂNG STELLAR SỬ DỤNG:
[ ] Chuyển XLM/USDC  [ ] Token tùy chỉnh  [x] Soroban contract
[ ] DEX tích hợp    [ ] Trustline     [ ] Clawback/Tuân thủ

NGƯỜI DÙNG MỤC TIÊU:
Freelancer, người thuê dịch vụ, marketplace nhỏ, hoặc developer muốn học cách xây dựng escrow logic trên blockchain.

TÍNH NĂNG CỐT LÕI (MVP):
Người dùng tạo escrow bằng init(payer, payee, amount), sau đó có thể gọi release() để hoàn tất giao dịch hoặc refund() để hủy và hoàn tiền.

TẠI SAO STELLAR:
Stellar cho phép thực hiện giao dịch với chi phí thấp và tốc độ nhanh, giúp các ứng dụng escrow hoạt động hiệu quả hơn so với hệ thống tài chính truyền thống vốn chậm và phụ thuộc bên trung gian.
