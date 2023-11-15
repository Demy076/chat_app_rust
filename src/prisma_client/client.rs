#![allow(warnings)]
pub static DATAMODEL_STR: &'static str = include_str!("/mnt/nvme/rust_tests/prisma/schema.prisma");
static DATABASE_STR: &'static str = "mysql";
pub async fn new_client() -> Result<PrismaClient, ::prisma_client_rust::NewClientError> {
    PrismaClient::_builder().build().await
}
pub async fn new_client_with_url(
    url: &str,
) -> Result<PrismaClient, ::prisma_client_rust::NewClientError> {
    PrismaClient::_builder()
        .with_url(url.to_string())
        .build()
        .await
}
pub mod user {
    use super::_prisma::*;
    use super::*;
    pub const NAME: &str = "User";
    pub mod id {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "id";
        pub struct Set(pub i32);
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetId(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::Id(v)
            }
        }
        pub fn set<T: From<Set>>(value: i32) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::Id(direction)
        }
        pub fn equals<T: From<UniqueWhereParam>>(value: i32) -> T {
            UniqueWhereParam::IdEquals(value).into()
        }
        ::prisma_client_rust::scalar_where_param_fns!(_prisma::read_filters::IntFilter, Id, {
            fn in_vec(_: Vec<i32>) -> InVec;
            fn not_in_vec(_: Vec<i32>) -> NotInVec;
            fn lt(_: i32) -> Lt;
            fn lte(_: i32) -> Lte;
            fn gt(_: i32) -> Gt;
            fn gte(_: i32) -> Gte;
            fn not(_: i32) -> Not;
        });
        pub fn increment(value: i32) -> SetParam {
            SetParam::IncrementId(value)
        }
        pub fn decrement(value: i32) -> SetParam {
            SetParam::DecrementId(value)
        }
        pub fn multiply(value: i32) -> SetParam {
            SetParam::MultiplyId(value)
        }
        pub fn divide(value: i32) -> SetParam {
            SetParam::DivideId(value)
        }
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::Id(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::Id(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod email {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "email";
        pub struct Set(pub String);
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetEmail(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::Email(v)
            }
        }
        pub fn set<T: From<Set>>(value: String) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::Email(direction)
        }
        pub fn equals<T: From<UniqueWhereParam>>(value: String) -> T {
            UniqueWhereParam::EmailEquals(value).into()
        }
        ::prisma_client_rust::scalar_where_param_fns!(
            _prisma::read_filters::StringFilter,
            Email,
            {
                fn in_vec(_: Vec<String>) -> InVec;
                fn not_in_vec(_: Vec<String>) -> NotInVec;
                fn lt(_: String) -> Lt;
                fn lte(_: String) -> Lte;
                fn gt(_: String) -> Gt;
                fn gte(_: String) -> Gte;
                fn contains(_: String) -> Contains;
                fn starts_with(_: String) -> StartsWith;
                fn ends_with(_: String) -> EndsWith;
                fn not(_: String) -> Not;
            }
        );
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::Email(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::Email(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod name {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "name";
        pub struct Set(pub Option<String>);
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetName(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::Name(v)
            }
        }
        pub fn set<T: From<Set>>(value: Option<String>) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::Name(direction)
        }
        pub fn equals<A, T: ::prisma_client_rust::FromOptionalUniqueArg<Set, Arg = A>>(
            value: A,
        ) -> T {
            T::from_arg(value)
        }
        ::prisma_client_rust::scalar_where_param_fns!(
            _prisma::read_filters::StringNullableFilter,
            Name,
            {
                fn in_vec(_: Vec<String>) -> InVec;
                fn not_in_vec(_: Vec<String>) -> NotInVec;
                fn lt(_: String) -> Lt;
                fn lte(_: String) -> Lte;
                fn gt(_: String) -> Gt;
                fn gte(_: String) -> Gte;
                fn contains(_: String) -> Contains;
                fn starts_with(_: String) -> StartsWith;
                fn ends_with(_: String) -> EndsWith;
                fn not(_: Option<String>) -> Not;
            }
        );
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::Name(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::Name(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod ip {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "ip";
        pub struct Set(pub String);
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetIp(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::Ip(v)
            }
        }
        pub fn set<T: From<Set>>(value: String) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::Ip(direction)
        }
        pub fn equals(value: String) -> WhereParam {
            WhereParam::Ip(_prisma::read_filters::StringFilter::Equals(value))
        }
        ::prisma_client_rust::scalar_where_param_fns!(_prisma::read_filters::StringFilter, Ip, {
            fn in_vec(_: Vec<String>) -> InVec;
            fn not_in_vec(_: Vec<String>) -> NotInVec;
            fn lt(_: String) -> Lt;
            fn lte(_: String) -> Lte;
            fn gt(_: String) -> Gt;
            fn gte(_: String) -> Gte;
            fn contains(_: String) -> Contains;
            fn starts_with(_: String) -> StartsWith;
            fn ends_with(_: String) -> EndsWith;
            fn not(_: String) -> Not;
        });
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::Ip(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::Ip(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod created_at {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "createdAt";
        pub struct Set(
            pub ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        );
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetCreatedAt(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::CreatedAt(v)
            }
        }
        pub fn set<T: From<Set>>(
            value: ::prisma_client_rust::chrono::DateTime<
                ::prisma_client_rust::chrono::FixedOffset,
            >,
        ) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::CreatedAt(direction)
        }
        pub fn equals(
            value: ::prisma_client_rust::chrono::DateTime<
                ::prisma_client_rust::chrono::FixedOffset,
            >,
        ) -> WhereParam {
            WhereParam::CreatedAt(_prisma::read_filters::DateTimeFilter::Equals(value))
        }
        ::prisma_client_rust::scalar_where_param_fns!(
            _prisma::read_filters::DateTimeFilter,
            CreatedAt,
            {
                fn in_vec(
                    _: Vec<
                        ::prisma_client_rust::chrono::DateTime<
                            ::prisma_client_rust::chrono::FixedOffset,
                        >,
                    >,
                ) -> InVec;
                fn not_in_vec(
                    _: Vec<
                        ::prisma_client_rust::chrono::DateTime<
                            ::prisma_client_rust::chrono::FixedOffset,
                        >,
                    >,
                ) -> NotInVec;
                fn lt(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Lt;
                fn lte(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Lte;
                fn gt(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Gt;
                fn gte(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Gte;
                fn not(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Not;
            }
        );
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::CreatedAt(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::CreatedAt(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod updated_at {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "updatedAt";
        pub struct Set(
            pub ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        );
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetUpdatedAt(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::UpdatedAt(v)
            }
        }
        pub fn set<T: From<Set>>(
            value: ::prisma_client_rust::chrono::DateTime<
                ::prisma_client_rust::chrono::FixedOffset,
            >,
        ) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::UpdatedAt(direction)
        }
        pub fn equals(
            value: ::prisma_client_rust::chrono::DateTime<
                ::prisma_client_rust::chrono::FixedOffset,
            >,
        ) -> WhereParam {
            WhereParam::UpdatedAt(_prisma::read_filters::DateTimeFilter::Equals(value))
        }
        ::prisma_client_rust::scalar_where_param_fns!(
            _prisma::read_filters::DateTimeFilter,
            UpdatedAt,
            {
                fn in_vec(
                    _: Vec<
                        ::prisma_client_rust::chrono::DateTime<
                            ::prisma_client_rust::chrono::FixedOffset,
                        >,
                    >,
                ) -> InVec;
                fn not_in_vec(
                    _: Vec<
                        ::prisma_client_rust::chrono::DateTime<
                            ::prisma_client_rust::chrono::FixedOffset,
                        >,
                    >,
                ) -> NotInVec;
                fn lt(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Lt;
                fn lte(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Lte;
                fn gt(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Gt;
                fn gte(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Gte;
                fn not(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Not;
            }
        );
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::UpdatedAt(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::UpdatedAt(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod messages {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "Messages";
        pub struct Fetch(pub messages::ManyArgs);
        impl Fetch {
            pub fn with(mut self, params: impl Into<messages::WithParam>) -> Self {
                self.0 = self.0.with(params.into());
                self
            }
            pub fn order_by(mut self, param: messages::OrderByParam) -> Self {
                self.0 = self.0.order_by(param);
                self
            }
            pub fn skip(mut self, value: i64) -> Self {
                self.0 = self.0.skip(value);
                self
            }
            pub fn take(mut self, value: i64) -> Self {
                self.0 = self.0.take(value);
                self
            }
            pub fn cursor(mut self, value: messages::UniqueWhereParam) -> Self {
                self.0 = self.0.cursor(value.into());
                self
            }
        }
        impl From<Fetch> for WithParam {
            fn from(Fetch(v): Fetch) -> Self {
                WithParam::Messages(v)
            }
        }
        pub fn fetch(params: Vec<messages::WhereParam>) -> Fetch {
            Fetch(messages::ManyArgs::new(params))
        }
        pub struct Connect(pub Vec<messages::UniqueWhereParam>);
        impl From<Connect> for SetParam {
            fn from(Connect(v): Connect) -> Self {
                Self::ConnectMessages(v)
            }
        }
        pub fn connect<T: From<Connect>>(params: Vec<messages::UniqueWhereParam>) -> T {
            Connect(params).into()
        }
        pub fn disconnect(params: Vec<messages::UniqueWhereParam>) -> SetParam {
            SetParam::DisconnectMessages(params)
        }
        pub fn set(params: Vec<messages::UniqueWhereParam>) -> SetParam {
            SetParam::SetMessages(params)
        }
        pub fn some(value: Vec<messages::WhereParam>) -> WhereParam {
            WhereParam::MessagesSome(value)
        }
        pub fn every(value: Vec<messages::WhereParam>) -> WhereParam {
            WhereParam::MessagesEvery(value)
        }
        pub fn none(value: Vec<messages::WhereParam>) -> WhereParam {
            WhereParam::MessagesNone(value)
        }
        pub enum Include {
            Select(messages::ManyArgs, Vec<messages::SelectParam>),
            Include(messages::ManyArgs, Vec<messages::IncludeParam>),
            Fetch(messages::ManyArgs),
        }
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::Messages(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                let (args, selections) = match self {
                    Self::Select(args, selections) => (
                        args.to_graphql().0,
                        selections.into_iter().map(|s| s.to_selection()).collect(),
                    ),
                    Self::Include(args, selections) => (args.to_graphql().0, {
                        let mut nested_selections = < messages :: Types as :: prisma_client_rust :: ModelTypes > :: scalar_selections () ;
                        nested_selections.extend(selections.into_iter().map(|s| s.to_selection()));
                        nested_selections
                    }),
                    Self::Fetch(args) => (
                        args.to_graphql().0,
                        <messages::Types as ::prisma_client_rust::ModelTypes>::scalar_selections(),
                    ),
                };
                ::prisma_client_rust::Selection::new(NAME, None, args, selections)
            }
            pub fn select(
                args: messages::ManyArgs,
                nested_selections: Vec<messages::SelectParam>,
            ) -> Self {
                Self::Select(args, nested_selections)
            }
            pub fn include(
                args: messages::ManyArgs,
                nested_selections: Vec<messages::IncludeParam>,
            ) -> Self {
                Self::Include(args, nested_selections)
            }
        }
        pub enum Select {
            Select(messages::ManyArgs, Vec<messages::SelectParam>),
            Include(messages::ManyArgs, Vec<messages::IncludeParam>),
            Fetch(messages::ManyArgs),
        }
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::Messages(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                let (args, selections) = match self {
                    Self::Select(args, selections) => (
                        args.to_graphql().0,
                        selections.into_iter().map(|s| s.to_selection()).collect(),
                    ),
                    Self::Include(args, selections) => (args.to_graphql().0, {
                        let mut nested_selections = vec![];
                        nested_selections.extend(selections.into_iter().map(|s| s.to_selection()));
                        nested_selections
                    }),
                    Self::Fetch(args) => (
                        args.to_graphql().0,
                        <messages::Types as ::prisma_client_rust::ModelTypes>::scalar_selections(),
                    ),
                };
                ::prisma_client_rust::Selection::new(NAME, None, args, selections)
            }
            pub fn select(
                args: messages::ManyArgs,
                nested_selections: Vec<messages::SelectParam>,
            ) -> Self {
                Self::Select(args, nested_selections)
            }
            pub fn include(
                args: messages::ManyArgs,
                nested_selections: Vec<messages::IncludeParam>,
            ) -> Self {
                Self::Include(args, nested_selections)
            }
        }
    }
    pub fn create(
        email: String,
        ip: String,
        _params: Vec<SetParam>,
    ) -> (String, String, Vec<SetParam>) {
        (email, ip, _params)
    }
    pub fn create_unchecked(
        email: String,
        ip: String,
        _params: Vec<SetParam>,
    ) -> (String, String, Vec<SetParam>) {
        (email, ip, _params)
    }
    #[macro_export]
    macro_rules ! _select_user { ($ (($ ($ func_arg : ident : $ func_arg_ty : ty) , +) =>) ? $ module_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { # [allow (warnings)] pub mod $ module_name { crate :: prisma :: user :: select ! (@ definitions ; $ module_name ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; use super :: * ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: SelectType for Selection { type Data = Data ; type ModelData = crate :: prisma :: user :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } pub fn select ($ ($ ($ func_arg : $ func_arg_ty) , +) ?) -> Selection { Selection ([crate :: prisma :: user :: select ! (@ selections_to_params ; : select { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () ,] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } } ; ({ $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { { crate :: prisma :: user :: select ! (@ definitions ; ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: SelectType for Selection { type Data = Data ; type ModelData = crate :: prisma :: user :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } Selection ([crate :: prisma :: user :: select ! (@ selections_to_params ; : select { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () ,] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } ; (@ definitions ; $ ($ module_name : ident) ? ; $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) +) => { # [allow (warnings)] enum Fields { id , email , name , ip , created_at , updated_at , messages } # [allow (warnings)] impl Fields { fn selections () { $ (let _ = Fields :: $ field ;) + } } # [allow (warnings)] # [derive (std :: fmt :: Debug , Clone)] pub struct Data { $ (pub $ field : crate :: prisma :: user :: select ! (@ field_type ; $ field $ (: $ selection_mode { $ ($ selections) + }) ?) ,) + } impl :: serde :: Serialize for Data { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : :: serde :: Serializer , { use :: serde :: ser :: SerializeStruct ; let mut state = serializer . serialize_struct ("Data" , [$ (stringify ! ($ field) ,) +] . len ()) ? ; $ (state . serialize_field (crate :: prisma :: user :: $ field :: NAME , & self . $ field) ? ;) * state . end () } } impl < 'de > :: serde :: Deserialize < 'de > for Data { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : :: serde :: Deserializer < 'de > , { # [allow (warnings)] enum Field { $ ($ field) , + , } impl < 'de > :: serde :: Deserialize < 'de > for Field { fn deserialize < D > (deserializer : D) -> Result < Field , D :: Error > where D : :: serde :: Deserializer < 'de > , { struct FieldVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for FieldVisitor { type Value = Field ; fn expecting (& self , formatter : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { formatter . write_str (& [$ (crate :: prisma :: user :: $ field :: NAME) , + ,] . into_iter () . collect :: < Vec < _ >> () . join (", ")) } fn visit_str < E > (self , value : & str) -> Result < Field , E > where E : :: serde :: de :: Error , { match value { $ (crate :: prisma :: user :: $ field :: NAME => Ok (Field :: $ field)) , * , _ => Err (:: serde :: de :: Error :: unknown_field (value , FIELDS)) , } } } deserializer . deserialize_identifier (FieldVisitor) } } struct DataVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for DataVisitor { type Value = Data ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("struct Data") } fn visit_map < V > (self , mut map : V) -> Result < Data , V :: Error > where V : :: serde :: de :: MapAccess < 'de > , { $ (let mut $ field = None ;) * while let Some (key) = map . next_key () ? { match key { $ (Field :: $ field => { if $ field . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: user :: $ field :: NAME)) ; } $ field = Some (map . next_value () ?) ; }) * } } $ (let $ field = $ field . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: user :: $ field :: NAME)) ? ;) * Ok (Data { $ ($ field) , * }) } } const FIELDS : & 'static [& 'static str] = & ["id" , "email" , "name" , "ip" , "createdAt" , "updatedAt" , "Messages"] ; deserializer . deserialize_struct ("Data" , FIELDS , DataVisitor) } } $ ($ (pub mod $ field { crate :: prisma :: user :: $ selection_mode ! (@ field_module ; $ field : $ selection_mode { $ ($ selections) + }) ; }) ?) + } ; (@ field_type ; id) => { i32 } ; (@ field_type ; email) => { String } ; (@ field_type ; name) => { Option < String > } ; (@ field_type ; ip) => { String } ; (@ field_type ; created_at) => { :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > } ; (@ field_type ; updated_at) => { :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > } ; (@ field_type ; messages : $ selection_mode : ident { $ ($ selections : tt) + }) => { Vec < messages :: Data > } ; (@ field_type ; messages) => { Vec < crate :: prisma :: messages :: Data > } ; (@ field_type ; $ field : ident $ ($ tokens : tt) *) => { compile_error ! (stringify ! (Cannot include nonexistent relation $ field on model "User" , available relations are "id, email, name, ip, created_at, updated_at, messages")) } ; (@ field_module ; messages : $ selection_mode : ident { $ ($ selections : tt) + }) => { crate :: prisma :: messages :: select ! (@ definitions ; ; $ ($ selections) +) ; } ; (@ field_module ; $ ($ tokens : tt) *) => { } ; (@ selection_field_to_selection_param ; id) => { Into :: < crate :: prisma :: user :: SelectParam > :: into (crate :: prisma :: user :: id :: Select) } ; (@ selection_field_to_selection_param ; email) => { Into :: < crate :: prisma :: user :: SelectParam > :: into (crate :: prisma :: user :: email :: Select) } ; (@ selection_field_to_selection_param ; name) => { Into :: < crate :: prisma :: user :: SelectParam > :: into (crate :: prisma :: user :: name :: Select) } ; (@ selection_field_to_selection_param ; ip) => { Into :: < crate :: prisma :: user :: SelectParam > :: into (crate :: prisma :: user :: ip :: Select) } ; (@ selection_field_to_selection_param ; created_at) => { Into :: < crate :: prisma :: user :: SelectParam > :: into (crate :: prisma :: user :: created_at :: Select) } ; (@ selection_field_to_selection_param ; updated_at) => { Into :: < crate :: prisma :: user :: SelectParam > :: into (crate :: prisma :: user :: updated_at :: Select) } ; (@ selection_field_to_selection_param ; messages $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? : $ selection_mode : ident { $ ($ selections : tt) + }) => { { Into :: < crate :: prisma :: user :: SelectParam > :: into (crate :: prisma :: user :: messages :: Select :: $ selection_mode (crate :: prisma :: messages :: ManyArgs :: new (crate :: prisma :: messages :: select ! (@ filters_to_args ; $ ($ ($ filters) +) ?)) $ ($ (. $ arg ($ ($ arg_params) *)) *) ? , crate :: prisma :: messages :: select ! (@ selections_to_params ; : $ selection_mode { $ ($ selections) + }) . into_iter () . collect ())) } } ; (@ selection_field_to_selection_param ; messages $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ?) => { { Into :: < crate :: prisma :: user :: SelectParam > :: into (crate :: prisma :: user :: messages :: Select :: Fetch (crate :: prisma :: messages :: ManyArgs :: new (crate :: prisma :: messages :: select ! (@ filters_to_args ; $ ($ ($ filters) +) ?)) $ ($ (. $ arg ($ ($ arg_params) *)) *) ?) ,) } } ; (@ selection_field_to_selection_param ; $ ($ tokens : tt) *) => { compile_error ! (stringify ! ($ ($ tokens) *)) } ; (@ selections_to_params ; : $ macro_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { [$ (crate :: prisma :: user :: $ macro_name ! (@ selection_field_to_selection_param ; $ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) ,) +] } ; (@ filters_to_args ;) => { vec ! [] } ; (@ filters_to_args ; $ ($ t : tt) *) => { $ ($ t) * } ; (@ field_serde_name ; id) => { "id" } ; (@ field_serde_name ; email) => { "email" } ; (@ field_serde_name ; name) => { "name" } ; (@ field_serde_name ; ip) => { "ip" } ; (@ field_serde_name ; created_at) => { "createdAt" } ; (@ field_serde_name ; updated_at) => { "updatedAt" } ; (@ field_serde_name ; messages) => { "Messages" } ; }
    pub use _select_user as select;
    pub enum SelectParam {
        Id(id::Select),
        Email(email::Select),
        Name(name::Select),
        Ip(ip::Select),
        CreatedAt(created_at::Select),
        UpdatedAt(updated_at::Select),
        Messages(messages::Select),
    }
    impl SelectParam {
        pub fn to_selection(self) -> ::prisma_client_rust::Selection {
            match self {
                Self::Id(data) => data.to_selection(),
                Self::Email(data) => data.to_selection(),
                Self::Name(data) => data.to_selection(),
                Self::Ip(data) => data.to_selection(),
                Self::CreatedAt(data) => data.to_selection(),
                Self::UpdatedAt(data) => data.to_selection(),
                Self::Messages(data) => data.to_selection(),
            }
        }
    }
    #[macro_export]
    macro_rules ! _include_user { ($ (($ ($ func_arg : ident : $ func_arg_ty : ty) , +) =>) ? $ module_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { # [allow (warnings)] pub mod $ module_name { crate :: prisma :: user :: include ! (@ definitions ; $ module_name ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; use super :: * ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: IncludeType for Selection { type Data = Data ; type ModelData = crate :: prisma :: user :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } pub fn include ($ ($ ($ func_arg : $ func_arg_ty) , +) ?) -> Selection { Selection ([crate :: prisma :: user :: include ! (@ selections_to_params ; : include { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () , < crate :: prisma :: user :: Types as :: prisma_client_rust :: ModelTypes > :: scalar_selections ()] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } } ; ({ $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { { crate :: prisma :: user :: include ! (@ definitions ; ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: IncludeType for Selection { type Data = Data ; type ModelData = crate :: prisma :: user :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } Selection ([crate :: prisma :: user :: include ! (@ selections_to_params ; : include { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () , < crate :: prisma :: user :: Types as :: prisma_client_rust :: ModelTypes > :: scalar_selections ()] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } ; (@ definitions ; $ ($ module_name : ident) ? ; $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) +) => { # [allow (warnings)] enum Fields { messages } # [allow (warnings)] impl Fields { fn selections () { $ (let _ = Fields :: $ field ;) + } } # [allow (warnings)] # [derive (std :: fmt :: Debug , Clone)] pub struct Data { pub id : i32 , pub email : String , pub name : Option < String > , pub ip : String , pub created_at : :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > , pub updated_at : :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > , $ (pub $ field : crate :: prisma :: user :: include ! (@ field_type ; $ field $ (: $ selection_mode { $ ($ selections) + }) ?) ,) + } impl :: serde :: Serialize for Data { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : :: serde :: Serializer , { use :: serde :: ser :: SerializeStruct ; let mut state = serializer . serialize_struct ("Data" , [$ (stringify ! ($ field) ,) + stringify ! (id) , stringify ! (email) , stringify ! (name) , stringify ! (ip) , stringify ! (created_at) , stringify ! (updated_at)] . len ()) ? ; $ (state . serialize_field (crate :: prisma :: user :: $ field :: NAME , & self . $ field) ? ;) * state . serialize_field (crate :: prisma :: user :: id :: NAME , & self . id) ? ; state . serialize_field (crate :: prisma :: user :: email :: NAME , & self . email) ? ; state . serialize_field (crate :: prisma :: user :: name :: NAME , & self . name) ? ; state . serialize_field (crate :: prisma :: user :: ip :: NAME , & self . ip) ? ; state . serialize_field (crate :: prisma :: user :: created_at :: NAME , & self . created_at) ? ; state . serialize_field (crate :: prisma :: user :: updated_at :: NAME , & self . updated_at) ? ; state . end () } } impl < 'de > :: serde :: Deserialize < 'de > for Data { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : :: serde :: Deserializer < 'de > , { # [allow (warnings)] enum Field { $ ($ field) , + , id , email , name , ip , created_at , updated_at } impl < 'de > :: serde :: Deserialize < 'de > for Field { fn deserialize < D > (deserializer : D) -> Result < Field , D :: Error > where D : :: serde :: Deserializer < 'de > , { struct FieldVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for FieldVisitor { type Value = Field ; fn expecting (& self , formatter : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { formatter . write_str (& [$ (crate :: prisma :: user :: $ field :: NAME) , + , crate :: prisma :: user :: id :: NAME , crate :: prisma :: user :: email :: NAME , crate :: prisma :: user :: name :: NAME , crate :: prisma :: user :: ip :: NAME , crate :: prisma :: user :: created_at :: NAME , crate :: prisma :: user :: updated_at :: NAME] . into_iter () . collect :: < Vec < _ >> () . join (", ")) } fn visit_str < E > (self , value : & str) -> Result < Field , E > where E : :: serde :: de :: Error , { match value { $ (crate :: prisma :: user :: $ field :: NAME => Ok (Field :: $ field)) , * , crate :: prisma :: user :: id :: NAME => Ok (Field :: id) , crate :: prisma :: user :: email :: NAME => Ok (Field :: email) , crate :: prisma :: user :: name :: NAME => Ok (Field :: name) , crate :: prisma :: user :: ip :: NAME => Ok (Field :: ip) , crate :: prisma :: user :: created_at :: NAME => Ok (Field :: created_at) , crate :: prisma :: user :: updated_at :: NAME => Ok (Field :: updated_at) , _ => Err (:: serde :: de :: Error :: unknown_field (value , FIELDS)) , } } } deserializer . deserialize_identifier (FieldVisitor) } } struct DataVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for DataVisitor { type Value = Data ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("struct Data") } fn visit_map < V > (self , mut map : V) -> Result < Data , V :: Error > where V : :: serde :: de :: MapAccess < 'de > , { $ (let mut $ field = None ;) * let mut id = None ; let mut email = None ; let mut name = None ; let mut ip = None ; let mut created_at = None ; let mut updated_at = None ; while let Some (key) = map . next_key () ? { match key { Field :: id => { if id . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: user :: id :: NAME)) ; } id = Some (map . next_value () ?) ; } Field :: email => { if email . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: user :: email :: NAME)) ; } email = Some (map . next_value () ?) ; } Field :: name => { if name . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: user :: name :: NAME)) ; } name = Some (map . next_value () ?) ; } Field :: ip => { if ip . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: user :: ip :: NAME)) ; } ip = Some (map . next_value () ?) ; } Field :: created_at => { if created_at . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: user :: created_at :: NAME)) ; } created_at = Some (map . next_value () ?) ; } Field :: updated_at => { if updated_at . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: user :: updated_at :: NAME)) ; } updated_at = Some (map . next_value () ?) ; } $ (Field :: $ field => { if $ field . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: user :: $ field :: NAME)) ; } $ field = Some (map . next_value () ?) ; }) * } } $ (let $ field = $ field . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: user :: $ field :: NAME)) ? ;) * let id = id . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: user :: id :: NAME)) ? ; let email = email . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: user :: email :: NAME)) ? ; let name = name . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: user :: name :: NAME)) ? ; let ip = ip . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: user :: ip :: NAME)) ? ; let created_at = created_at . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: user :: created_at :: NAME)) ? ; let updated_at = updated_at . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: user :: updated_at :: NAME)) ? ; Ok (Data { id , email , name , ip , created_at , updated_at , $ ($ field) , * }) } } const FIELDS : & 'static [& 'static str] = & ["id" , "email" , "name" , "ip" , "createdAt" , "updatedAt" , "Messages"] ; deserializer . deserialize_struct ("Data" , FIELDS , DataVisitor) } } $ ($ (pub mod $ field { crate :: prisma :: user :: $ selection_mode ! (@ field_module ; $ field : $ selection_mode { $ ($ selections) + }) ; }) ?) + } ; (@ field_type ; messages : $ selection_mode : ident { $ ($ selections : tt) + }) => { Vec < messages :: Data > } ; (@ field_type ; messages) => { Vec < crate :: prisma :: messages :: Data > } ; (@ field_type ; $ field : ident $ ($ tokens : tt) *) => { compile_error ! (stringify ! (Cannot include nonexistent relation $ field on model "User" , available relations are "messages")) } ; (@ field_module ; messages : $ selection_mode : ident { $ ($ selections : tt) + }) => { crate :: prisma :: messages :: include ! (@ definitions ; ; $ ($ selections) +) ; } ; (@ field_module ; $ ($ tokens : tt) *) => { } ; (@ selection_field_to_selection_param ; messages $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? : $ selection_mode : ident { $ ($ selections : tt) + }) => { { Into :: < crate :: prisma :: user :: IncludeParam > :: into (crate :: prisma :: user :: messages :: Include :: $ selection_mode (crate :: prisma :: messages :: ManyArgs :: new (crate :: prisma :: messages :: include ! (@ filters_to_args ; $ ($ ($ filters) +) ?)) $ ($ (. $ arg ($ ($ arg_params) *)) *) ? , crate :: prisma :: messages :: select ! (@ selections_to_params ; : $ selection_mode { $ ($ selections) + }) . into_iter () . collect ())) } } ; (@ selection_field_to_selection_param ; messages $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ?) => { { Into :: < crate :: prisma :: user :: IncludeParam > :: into (crate :: prisma :: user :: messages :: Include :: Fetch (crate :: prisma :: messages :: ManyArgs :: new (crate :: prisma :: messages :: include ! (@ filters_to_args ; $ ($ ($ filters) +) ?)) $ ($ (. $ arg ($ ($ arg_params) *)) *) ?) ,) } } ; (@ selection_field_to_selection_param ; $ ($ tokens : tt) *) => { compile_error ! (stringify ! ($ ($ tokens) *)) } ; (@ selections_to_params ; : $ macro_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { [$ (crate :: prisma :: user :: $ macro_name ! (@ selection_field_to_selection_param ; $ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) ,) +] } ; (@ filters_to_args ;) => { vec ! [] } ; (@ filters_to_args ; $ ($ t : tt) *) => { $ ($ t) * } ; (@ field_serde_name ; id) => { "id" } ; (@ field_serde_name ; email) => { "email" } ; (@ field_serde_name ; name) => { "name" } ; (@ field_serde_name ; ip) => { "ip" } ; (@ field_serde_name ; created_at) => { "createdAt" } ; (@ field_serde_name ; updated_at) => { "updatedAt" } ; (@ field_serde_name ; messages) => { "Messages" } ; }
    pub use _include_user as include;
    pub enum IncludeParam {
        Id(id::Include),
        Email(email::Include),
        Name(name::Include),
        Ip(ip::Include),
        CreatedAt(created_at::Include),
        UpdatedAt(updated_at::Include),
        Messages(messages::Include),
    }
    impl IncludeParam {
        pub fn to_selection(self) -> ::prisma_client_rust::Selection {
            match self {
                Self::Id(data) => data.to_selection(),
                Self::Email(data) => data.to_selection(),
                Self::Name(data) => data.to_selection(),
                Self::Ip(data) => data.to_selection(),
                Self::CreatedAt(data) => data.to_selection(),
                Self::UpdatedAt(data) => data.to_selection(),
                Self::Messages(data) => data.to_selection(),
            }
        }
    }
    #[macro_export]
    macro_rules ! _partial_unchecked_user { ($ struct_name : ident { $ ($ scalar_field : ident) + }) => { :: prisma_client_rust :: macros :: partial_unchecked ! { crate :: prisma :: user struct $ struct_name { # [serde (rename = "id")] pub id : i32 , # [serde (rename = "email")] pub email : String , # [serde (rename = "name")] # [serde (default , with = "::prisma_client_rust::serde::double_option")] pub name : Option < String > , # [serde (rename = "ip")] pub ip : String , # [serde (rename = "createdAt")] pub created_at : :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > , # [serde (rename = "updatedAt")] pub updated_at : :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > } [$ ($ scalar_field) , +] } } ; }
    pub use _partial_unchecked_user as partial_unchecked;
    #[derive(Debug, Clone, :: serde :: Serialize, :: serde :: Deserialize)]
    pub struct Data {
        #[serde(rename = "id")]
        pub id: i32,
        #[serde(rename = "email")]
        pub email: String,
        #[serde(rename = "name")]
        pub name: Option<String>,
        #[serde(rename = "ip")]
        pub ip: String,
        #[serde(rename = "createdAt")]
        pub created_at:
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        #[serde(rename = "updatedAt")]
        pub updated_at:
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        #[serde(rename = "Messages")]
        pub messages: Option<Vec<super::messages::Data>>,
    }
    impl Data {
        pub fn messages(
            &self,
        ) -> Result<&Vec<super::messages::Data>, ::prisma_client_rust::RelationNotFetchedError>
        {
            self.messages
                .as_ref()
                .ok_or(::prisma_client_rust::RelationNotFetchedError::new(
                    stringify!(messages),
                ))
        }
    }
    #[derive(Clone)]
    pub enum WithParam {
        Messages(super::messages::ManyArgs),
    }
    impl Into<::prisma_client_rust::Selection> for WithParam {
        fn into(self) -> ::prisma_client_rust::Selection {
            match self {
                Self::Messages(args) => {
                    let (arguments, mut nested_selections) = args.to_graphql();
                    nested_selections . extend (< super :: messages :: Types as :: prisma_client_rust :: ModelTypes > :: scalar_selections ()) ;
                    ::prisma_client_rust::Selection::new(
                        messages::NAME,
                        None,
                        arguments,
                        nested_selections,
                    )
                }
            }
        }
    }
    #[derive(Clone)]
    pub enum SetParam {
        SetId(i32),
        IncrementId(i32),
        DecrementId(i32),
        MultiplyId(i32),
        DivideId(i32),
        SetEmail(String),
        SetName(Option<String>),
        SetIp(String),
        SetCreatedAt(
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        ),
        SetUpdatedAt(
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        ),
        ConnectMessages(Vec<super::messages::UniqueWhereParam>),
        DisconnectMessages(Vec<super::messages::UniqueWhereParam>),
        SetMessages(Vec<super::messages::UniqueWhereParam>),
    }
    impl From<SetParam> for (String, ::prisma_client_rust::PrismaValue) {
        fn from(param: SetParam) -> Self {
            match param {
                SetParam::SetId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Int(value as i64),
                ),
                SetParam::IncrementId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "increment".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::DecrementId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "decrement".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::MultiplyId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "multiply".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::DivideId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "divide".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::SetEmail(value) => (
                    email::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(value),
                ),
                SetParam::SetName(value) => (
                    name::NAME.to_string(),
                    value
                        .map(|value| ::prisma_client_rust::PrismaValue::String(value))
                        .unwrap_or_else(|| ::prisma_client_rust::PrismaValue::Null),
                ),
                SetParam::SetIp(value) => (
                    ip::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(value),
                ),
                SetParam::SetCreatedAt(value) => (
                    created_at::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::DateTime(value),
                ),
                SetParam::SetUpdatedAt(value) => (
                    updated_at::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::DateTime(value),
                ),
                SetParam::ConnectMessages(where_params) => (
                    messages::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "connect".to_string(),
                        ::prisma_client_rust::PrismaValue::List(
                            where_params
                                .into_iter()
                                .map(Into::<super::messages::WhereParam>::into)
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .map(|v| ::prisma_client_rust::PrismaValue::Object(vec![v]))
                                .collect(),
                        ),
                    )]),
                ),
                SetParam::DisconnectMessages(where_params) => (
                    messages::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "disconnect".to_string(),
                        ::prisma_client_rust::PrismaValue::List(
                            where_params
                                .into_iter()
                                .map(Into::<super::messages::WhereParam>::into)
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .map(|v| ::prisma_client_rust::PrismaValue::Object(vec![v]))
                                .collect(),
                        ),
                    )]),
                ),
                SetParam::SetMessages(where_params) => (
                    messages::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "set".to_string(),
                        ::prisma_client_rust::PrismaValue::List(
                            where_params
                                .into_iter()
                                .map(Into::<super::messages::WhereParam>::into)
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .map(|v| ::prisma_client_rust::PrismaValue::Object(vec![v]))
                                .collect(),
                        ),
                    )]),
                ),
            }
        }
    }
    #[derive(Clone)]
    pub enum UncheckedSetParam {
        Id(i32),
        Email(String),
        Name(Option<String>),
        Ip(String),
        CreatedAt(
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        ),
        UpdatedAt(
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        ),
    }
    impl From<UncheckedSetParam> for SetParam {
        fn from(param: UncheckedSetParam) -> Self {
            match param {
                UncheckedSetParam::Id(value) => Self::SetId(value),
                UncheckedSetParam::Email(value) => Self::SetEmail(value),
                UncheckedSetParam::Name(value) => Self::SetName(value),
                UncheckedSetParam::Ip(value) => Self::SetIp(value),
                UncheckedSetParam::CreatedAt(value) => Self::SetCreatedAt(value),
                UncheckedSetParam::UpdatedAt(value) => Self::SetUpdatedAt(value),
            }
        }
    }
    #[derive(Clone)]
    pub enum OrderByParam {
        Id(::prisma_client_rust::Direction),
        Email(::prisma_client_rust::Direction),
        Name(::prisma_client_rust::Direction),
        Ip(::prisma_client_rust::Direction),
        CreatedAt(::prisma_client_rust::Direction),
        UpdatedAt(::prisma_client_rust::Direction),
    }
    impl Into<(String, ::prisma_client_rust::PrismaValue)> for OrderByParam {
        fn into(self) -> (String, ::prisma_client_rust::PrismaValue) {
            match self {
                Self::Id(direction) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
                Self::Email(direction) => (
                    email::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
                Self::Name(direction) => (
                    name::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
                Self::Ip(direction) => (
                    ip::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
                Self::CreatedAt(direction) => (
                    created_at::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
                Self::UpdatedAt(direction) => (
                    updated_at::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
            }
        }
    }
    #[derive(Clone)]
    pub enum WhereParam {
        Not(Vec<WhereParam>),
        Or(Vec<WhereParam>),
        And(Vec<WhereParam>),
        Id(_prisma::read_filters::IntFilter),
        Email(_prisma::read_filters::StringFilter),
        Name(_prisma::read_filters::StringNullableFilter),
        Ip(_prisma::read_filters::StringFilter),
        CreatedAt(_prisma::read_filters::DateTimeFilter),
        UpdatedAt(_prisma::read_filters::DateTimeFilter),
        MessagesSome(Vec<super::messages::WhereParam>),
        MessagesEvery(Vec<super::messages::WhereParam>),
        MessagesNone(Vec<super::messages::WhereParam>),
    }
    impl ::prisma_client_rust::WhereInput for WhereParam {
        fn serialize(self) -> ::prisma_client_rust::SerializedWhereInput {
            let (name, value) = match self {
                Self::Not(value) => (
                    "NOT",
                    ::prisma_client_rust::SerializedWhereValue::Object(
                        ::prisma_client_rust::merge_fields(
                            value
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(Into::into)
                                .collect(),
                        ),
                    ),
                ),
                Self::Or(value) => (
                    "OR",
                    ::prisma_client_rust::SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(::prisma_client_rust::WhereInput::serialize)
                            .map(Into::into)
                            .map(|v| vec![v])
                            .map(::prisma_client_rust::PrismaValue::Object)
                            .collect(),
                    ),
                ),
                Self::And(value) => (
                    "AND",
                    ::prisma_client_rust::SerializedWhereValue::Object(
                        ::prisma_client_rust::merge_fields(
                            value
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(Into::into)
                                .collect(),
                        ),
                    ),
                ),
                Self::Id(value) => (id::NAME, value.into()),
                Self::Email(value) => (email::NAME, value.into()),
                Self::Name(value) => (name::NAME, value.into()),
                Self::Ip(value) => (ip::NAME, value.into()),
                Self::CreatedAt(value) => (created_at::NAME, value.into()),
                Self::UpdatedAt(value) => (updated_at::NAME, value.into()),
                Self::MessagesSome(where_params) => (
                    messages::NAME,
                    ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "some".to_string(),
                        ::prisma_client_rust::PrismaValue::Object(
                            where_params
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .collect(),
                        ),
                    )]),
                ),
                Self::MessagesEvery(where_params) => (
                    messages::NAME,
                    ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "every".to_string(),
                        ::prisma_client_rust::PrismaValue::Object(
                            where_params
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .collect(),
                        ),
                    )]),
                ),
                Self::MessagesNone(where_params) => (
                    messages::NAME,
                    ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "none".to_string(),
                        ::prisma_client_rust::PrismaValue::Object(
                            where_params
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .collect(),
                        ),
                    )]),
                ),
            };
            ::prisma_client_rust::SerializedWhereInput::new(name, value.into())
        }
    }
    #[derive(Clone)]
    pub enum UniqueWhereParam {
        EmailEquals(String),
        NameEquals(String),
        IdEquals(i32),
    }
    impl From<UniqueWhereParam> for WhereParam {
        fn from(value: UniqueWhereParam) -> Self {
            match value {
                UniqueWhereParam::EmailEquals(value) => {
                    Self::Email(_prisma::read_filters::StringFilter::Equals(value))
                }
                UniqueWhereParam::NameEquals(value) => Self::Name(
                    _prisma::read_filters::StringNullableFilter::Equals(Some(value)),
                ),
                UniqueWhereParam::IdEquals(value) => {
                    Self::Id(_prisma::read_filters::IntFilter::Equals(value))
                }
            }
        }
    }
    impl ::prisma_client_rust::FromOptionalUniqueArg<name::Set> for WhereParam {
        type Arg = Option<String>;
        fn from_arg(arg: Self::Arg) -> Self
        where
            Self: Sized,
        {
            Self::Name(_prisma::read_filters::StringNullableFilter::Equals(arg))
        }
    }
    impl ::prisma_client_rust::FromOptionalUniqueArg<name::Set> for UniqueWhereParam {
        type Arg = String;
        fn from_arg(arg: Self::Arg) -> Self
        where
            Self: Sized,
        {
            Self::NameEquals(arg)
        }
    }
    impl From<::prisma_client_rust::Operator<Self>> for WhereParam {
        fn from(op: ::prisma_client_rust::Operator<Self>) -> Self {
            match op {
                ::prisma_client_rust::Operator::Not(value) => Self::Not(value),
                ::prisma_client_rust::Operator::And(value) => Self::And(value),
                ::prisma_client_rust::Operator::Or(value) => Self::Or(value),
            }
        }
    }
    #[derive(Clone)]
    pub struct Types;
    impl ::prisma_client_rust::ModelTypes for Types {
        type Data = Data;
        type Where = WhereParam;
        type UncheckedSet = UncheckedSetParam;
        type Set = SetParam;
        type With = WithParam;
        type OrderBy = OrderByParam;
        type Cursor = UniqueWhereParam;
        const MODEL: &'static str = NAME;
        fn scalar_selections() -> Vec<::prisma_client_rust::Selection> {
            vec![
                ::prisma_client_rust::sel(id::NAME),
                ::prisma_client_rust::sel(email::NAME),
                ::prisma_client_rust::sel(name::NAME),
                ::prisma_client_rust::sel(ip::NAME),
                ::prisma_client_rust::sel(created_at::NAME),
                ::prisma_client_rust::sel(updated_at::NAME),
            ]
        }
    }
    pub type UniqueArgs = ::prisma_client_rust::UniqueArgs<Types>;
    pub type ManyArgs = ::prisma_client_rust::ManyArgs<Types>;
    pub type Count<'a> = ::prisma_client_rust::Count<'a, Types>;
    pub type Create<'a> = ::prisma_client_rust::Create<'a, Types>;
    pub type CreateMany<'a> = ::prisma_client_rust::CreateMany<'a, Types>;
    pub type FindUnique<'a> = ::prisma_client_rust::FindUnique<'a, Types>;
    pub type FindMany<'a> = ::prisma_client_rust::FindMany<'a, Types>;
    pub type FindFirst<'a> = ::prisma_client_rust::FindFirst<'a, Types>;
    pub type Update<'a> = ::prisma_client_rust::Update<'a, Types>;
    pub type UpdateMany<'a> = ::prisma_client_rust::UpdateMany<'a, Types>;
    pub type Upsert<'a> = ::prisma_client_rust::Upsert<'a, Types>;
    pub type Delete<'a> = ::prisma_client_rust::Delete<'a, Types>;
    pub type DeleteMany<'a> = ::prisma_client_rust::DeleteMany<'a, Types>;
    #[derive(Clone)]
    pub struct Actions<'a> {
        pub client: &'a ::prisma_client_rust::PrismaClientInternals,
    }
    impl<'a> Actions<'a> {
        pub fn find_unique(self, _where: UniqueWhereParam) -> FindUnique<'a> {
            FindUnique::new(self.client, _where.into())
        }
        pub fn find_first(self, _where: Vec<WhereParam>) -> FindFirst<'a> {
            FindFirst::new(self.client, _where)
        }
        pub fn find_many(self, _where: Vec<WhereParam>) -> FindMany<'a> {
            FindMany::new(self.client, _where)
        }
        pub fn create(self, email: String, ip: String, mut _params: Vec<SetParam>) -> Create<'a> {
            _params.extend([email::set(email), ip::set(ip)]);
            Create::new(self.client, _params)
        }
        pub fn create_unchecked(
            self,
            email: String,
            ip: String,
            mut _params: Vec<UncheckedSetParam>,
        ) -> Create<'a> {
            _params.extend([email::set(email), ip::set(ip)]);
            Create::new(self.client, _params.into_iter().map(Into::into).collect())
        }
        pub fn create_many(self, data: Vec<(String, String, Vec<SetParam>)>) -> CreateMany<'a> {
            let data = data
                .into_iter()
                .map(|(email, ip, mut _params)| {
                    _params.extend([email::set(email), ip::set(ip)]);
                    _params
                })
                .collect();
            CreateMany::new(self.client, data)
        }
        pub fn update(self, _where: UniqueWhereParam, _params: Vec<SetParam>) -> Update<'a> {
            Update::new(self.client, _where.into(), _params, vec![])
        }
        pub fn update_unchecked(
            self,
            _where: UniqueWhereParam,
            _params: Vec<UncheckedSetParam>,
        ) -> Update<'a> {
            Update::new(
                self.client,
                _where.into(),
                _params.into_iter().map(Into::into).collect(),
                vec![],
            )
        }
        pub fn update_many(
            self,
            _where: Vec<WhereParam>,
            _params: Vec<SetParam>,
        ) -> UpdateMany<'a> {
            UpdateMany::new(self.client, _where, _params)
        }
        pub fn upsert(
            self,
            _where: UniqueWhereParam,
            (email, ip, mut _params): (String, String, Vec<SetParam>),
            _update: Vec<SetParam>,
        ) -> Upsert<'a> {
            _params.extend([email::set(email), ip::set(ip)]);
            Upsert::new(self.client, _where.into(), _params, _update)
        }
        pub fn delete(self, _where: UniqueWhereParam) -> Delete<'a> {
            Delete::new(self.client, _where.into(), vec![])
        }
        pub fn delete_many(self, _where: Vec<WhereParam>) -> DeleteMany<'a> {
            DeleteMany::new(self.client, _where)
        }
        pub fn count(self, _where: Vec<WhereParam>) -> Count<'a> {
            Count::new(self.client, _where)
        }
        pub fn find_raw<T: ::prisma_client_rust::Data>(
            self,
        ) -> ::prisma_client_rust::FindRaw<'a, Types, T> {
            ::prisma_client_rust::FindRaw::new(self.client)
        }
        pub fn aggregate_raw<T: ::prisma_client_rust::Data>(
            self,
        ) -> ::prisma_client_rust::AggregateRaw<'a, Types, T> {
            ::prisma_client_rust::AggregateRaw::new(self.client)
        }
    }
}
pub mod rooms {
    use super::_prisma::*;
    use super::*;
    pub const NAME: &str = "Rooms";
    pub mod id {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "id";
        pub struct Set(pub i32);
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetId(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::Id(v)
            }
        }
        pub fn set<T: From<Set>>(value: i32) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::Id(direction)
        }
        pub fn equals<T: From<UniqueWhereParam>>(value: i32) -> T {
            UniqueWhereParam::IdEquals(value).into()
        }
        ::prisma_client_rust::scalar_where_param_fns!(_prisma::read_filters::IntFilter, Id, {
            fn in_vec(_: Vec<i32>) -> InVec;
            fn not_in_vec(_: Vec<i32>) -> NotInVec;
            fn lt(_: i32) -> Lt;
            fn lte(_: i32) -> Lte;
            fn gt(_: i32) -> Gt;
            fn gte(_: i32) -> Gte;
            fn not(_: i32) -> Not;
        });
        pub fn increment(value: i32) -> SetParam {
            SetParam::IncrementId(value)
        }
        pub fn decrement(value: i32) -> SetParam {
            SetParam::DecrementId(value)
        }
        pub fn multiply(value: i32) -> SetParam {
            SetParam::MultiplyId(value)
        }
        pub fn divide(value: i32) -> SetParam {
            SetParam::DivideId(value)
        }
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::Id(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::Id(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod name {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "name";
        pub struct Set(pub String);
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetName(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::Name(v)
            }
        }
        pub fn set<T: From<Set>>(value: String) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::Name(direction)
        }
        pub fn equals<T: From<UniqueWhereParam>>(value: String) -> T {
            UniqueWhereParam::NameEquals(value).into()
        }
        ::prisma_client_rust::scalar_where_param_fns!(_prisma::read_filters::StringFilter, Name, {
            fn in_vec(_: Vec<String>) -> InVec;
            fn not_in_vec(_: Vec<String>) -> NotInVec;
            fn lt(_: String) -> Lt;
            fn lte(_: String) -> Lte;
            fn gt(_: String) -> Gt;
            fn gte(_: String) -> Gte;
            fn contains(_: String) -> Contains;
            fn starts_with(_: String) -> StartsWith;
            fn ends_with(_: String) -> EndsWith;
            fn not(_: String) -> Not;
        });
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::Name(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::Name(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod created_at {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "createdAt";
        pub struct Set(
            pub ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        );
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetCreatedAt(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::CreatedAt(v)
            }
        }
        pub fn set<T: From<Set>>(
            value: ::prisma_client_rust::chrono::DateTime<
                ::prisma_client_rust::chrono::FixedOffset,
            >,
        ) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::CreatedAt(direction)
        }
        pub fn equals(
            value: ::prisma_client_rust::chrono::DateTime<
                ::prisma_client_rust::chrono::FixedOffset,
            >,
        ) -> WhereParam {
            WhereParam::CreatedAt(_prisma::read_filters::DateTimeFilter::Equals(value))
        }
        ::prisma_client_rust::scalar_where_param_fns!(
            _prisma::read_filters::DateTimeFilter,
            CreatedAt,
            {
                fn in_vec(
                    _: Vec<
                        ::prisma_client_rust::chrono::DateTime<
                            ::prisma_client_rust::chrono::FixedOffset,
                        >,
                    >,
                ) -> InVec;
                fn not_in_vec(
                    _: Vec<
                        ::prisma_client_rust::chrono::DateTime<
                            ::prisma_client_rust::chrono::FixedOffset,
                        >,
                    >,
                ) -> NotInVec;
                fn lt(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Lt;
                fn lte(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Lte;
                fn gt(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Gt;
                fn gte(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Gte;
                fn not(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Not;
            }
        );
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::CreatedAt(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::CreatedAt(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod updated_at {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "updatedAt";
        pub struct Set(
            pub ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        );
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetUpdatedAt(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::UpdatedAt(v)
            }
        }
        pub fn set<T: From<Set>>(
            value: ::prisma_client_rust::chrono::DateTime<
                ::prisma_client_rust::chrono::FixedOffset,
            >,
        ) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::UpdatedAt(direction)
        }
        pub fn equals(
            value: ::prisma_client_rust::chrono::DateTime<
                ::prisma_client_rust::chrono::FixedOffset,
            >,
        ) -> WhereParam {
            WhereParam::UpdatedAt(_prisma::read_filters::DateTimeFilter::Equals(value))
        }
        ::prisma_client_rust::scalar_where_param_fns!(
            _prisma::read_filters::DateTimeFilter,
            UpdatedAt,
            {
                fn in_vec(
                    _: Vec<
                        ::prisma_client_rust::chrono::DateTime<
                            ::prisma_client_rust::chrono::FixedOffset,
                        >,
                    >,
                ) -> InVec;
                fn not_in_vec(
                    _: Vec<
                        ::prisma_client_rust::chrono::DateTime<
                            ::prisma_client_rust::chrono::FixedOffset,
                        >,
                    >,
                ) -> NotInVec;
                fn lt(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Lt;
                fn lte(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Lte;
                fn gt(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Gt;
                fn gte(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Gte;
                fn not(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Not;
            }
        );
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::UpdatedAt(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::UpdatedAt(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod messages {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "Messages";
        pub struct Fetch(pub messages::ManyArgs);
        impl Fetch {
            pub fn with(mut self, params: impl Into<messages::WithParam>) -> Self {
                self.0 = self.0.with(params.into());
                self
            }
            pub fn order_by(mut self, param: messages::OrderByParam) -> Self {
                self.0 = self.0.order_by(param);
                self
            }
            pub fn skip(mut self, value: i64) -> Self {
                self.0 = self.0.skip(value);
                self
            }
            pub fn take(mut self, value: i64) -> Self {
                self.0 = self.0.take(value);
                self
            }
            pub fn cursor(mut self, value: messages::UniqueWhereParam) -> Self {
                self.0 = self.0.cursor(value.into());
                self
            }
        }
        impl From<Fetch> for WithParam {
            fn from(Fetch(v): Fetch) -> Self {
                WithParam::Messages(v)
            }
        }
        pub fn fetch(params: Vec<messages::WhereParam>) -> Fetch {
            Fetch(messages::ManyArgs::new(params))
        }
        pub struct Connect(pub Vec<messages::UniqueWhereParam>);
        impl From<Connect> for SetParam {
            fn from(Connect(v): Connect) -> Self {
                Self::ConnectMessages(v)
            }
        }
        pub fn connect<T: From<Connect>>(params: Vec<messages::UniqueWhereParam>) -> T {
            Connect(params).into()
        }
        pub fn disconnect(params: Vec<messages::UniqueWhereParam>) -> SetParam {
            SetParam::DisconnectMessages(params)
        }
        pub fn set(params: Vec<messages::UniqueWhereParam>) -> SetParam {
            SetParam::SetMessages(params)
        }
        pub fn some(value: Vec<messages::WhereParam>) -> WhereParam {
            WhereParam::MessagesSome(value)
        }
        pub fn every(value: Vec<messages::WhereParam>) -> WhereParam {
            WhereParam::MessagesEvery(value)
        }
        pub fn none(value: Vec<messages::WhereParam>) -> WhereParam {
            WhereParam::MessagesNone(value)
        }
        pub enum Include {
            Select(messages::ManyArgs, Vec<messages::SelectParam>),
            Include(messages::ManyArgs, Vec<messages::IncludeParam>),
            Fetch(messages::ManyArgs),
        }
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::Messages(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                let (args, selections) = match self {
                    Self::Select(args, selections) => (
                        args.to_graphql().0,
                        selections.into_iter().map(|s| s.to_selection()).collect(),
                    ),
                    Self::Include(args, selections) => (args.to_graphql().0, {
                        let mut nested_selections = < messages :: Types as :: prisma_client_rust :: ModelTypes > :: scalar_selections () ;
                        nested_selections.extend(selections.into_iter().map(|s| s.to_selection()));
                        nested_selections
                    }),
                    Self::Fetch(args) => (
                        args.to_graphql().0,
                        <messages::Types as ::prisma_client_rust::ModelTypes>::scalar_selections(),
                    ),
                };
                ::prisma_client_rust::Selection::new(NAME, None, args, selections)
            }
            pub fn select(
                args: messages::ManyArgs,
                nested_selections: Vec<messages::SelectParam>,
            ) -> Self {
                Self::Select(args, nested_selections)
            }
            pub fn include(
                args: messages::ManyArgs,
                nested_selections: Vec<messages::IncludeParam>,
            ) -> Self {
                Self::Include(args, nested_selections)
            }
        }
        pub enum Select {
            Select(messages::ManyArgs, Vec<messages::SelectParam>),
            Include(messages::ManyArgs, Vec<messages::IncludeParam>),
            Fetch(messages::ManyArgs),
        }
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::Messages(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                let (args, selections) = match self {
                    Self::Select(args, selections) => (
                        args.to_graphql().0,
                        selections.into_iter().map(|s| s.to_selection()).collect(),
                    ),
                    Self::Include(args, selections) => (args.to_graphql().0, {
                        let mut nested_selections = vec![];
                        nested_selections.extend(selections.into_iter().map(|s| s.to_selection()));
                        nested_selections
                    }),
                    Self::Fetch(args) => (
                        args.to_graphql().0,
                        <messages::Types as ::prisma_client_rust::ModelTypes>::scalar_selections(),
                    ),
                };
                ::prisma_client_rust::Selection::new(NAME, None, args, selections)
            }
            pub fn select(
                args: messages::ManyArgs,
                nested_selections: Vec<messages::SelectParam>,
            ) -> Self {
                Self::Select(args, nested_selections)
            }
            pub fn include(
                args: messages::ManyArgs,
                nested_selections: Vec<messages::IncludeParam>,
            ) -> Self {
                Self::Include(args, nested_selections)
            }
        }
    }
    pub fn create(name: String, _params: Vec<SetParam>) -> (String, Vec<SetParam>) {
        (name, _params)
    }
    pub fn create_unchecked(name: String, _params: Vec<SetParam>) -> (String, Vec<SetParam>) {
        (name, _params)
    }
    #[macro_export]
    macro_rules ! _select_rooms { ($ (($ ($ func_arg : ident : $ func_arg_ty : ty) , +) =>) ? $ module_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { # [allow (warnings)] pub mod $ module_name { crate :: prisma :: rooms :: select ! (@ definitions ; $ module_name ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; use super :: * ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: SelectType for Selection { type Data = Data ; type ModelData = crate :: prisma :: rooms :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } pub fn select ($ ($ ($ func_arg : $ func_arg_ty) , +) ?) -> Selection { Selection ([crate :: prisma :: rooms :: select ! (@ selections_to_params ; : select { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () ,] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } } ; ({ $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { { crate :: prisma :: rooms :: select ! (@ definitions ; ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: SelectType for Selection { type Data = Data ; type ModelData = crate :: prisma :: rooms :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } Selection ([crate :: prisma :: rooms :: select ! (@ selections_to_params ; : select { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () ,] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } ; (@ definitions ; $ ($ module_name : ident) ? ; $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) +) => { # [allow (warnings)] enum Fields { id , name , created_at , updated_at , messages } # [allow (warnings)] impl Fields { fn selections () { $ (let _ = Fields :: $ field ;) + } } # [allow (warnings)] # [derive (std :: fmt :: Debug , Clone)] pub struct Data { $ (pub $ field : crate :: prisma :: rooms :: select ! (@ field_type ; $ field $ (: $ selection_mode { $ ($ selections) + }) ?) ,) + } impl :: serde :: Serialize for Data { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : :: serde :: Serializer , { use :: serde :: ser :: SerializeStruct ; let mut state = serializer . serialize_struct ("Data" , [$ (stringify ! ($ field) ,) +] . len ()) ? ; $ (state . serialize_field (crate :: prisma :: rooms :: $ field :: NAME , & self . $ field) ? ;) * state . end () } } impl < 'de > :: serde :: Deserialize < 'de > for Data { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : :: serde :: Deserializer < 'de > , { # [allow (warnings)] enum Field { $ ($ field) , + , } impl < 'de > :: serde :: Deserialize < 'de > for Field { fn deserialize < D > (deserializer : D) -> Result < Field , D :: Error > where D : :: serde :: Deserializer < 'de > , { struct FieldVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for FieldVisitor { type Value = Field ; fn expecting (& self , formatter : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { formatter . write_str (& [$ (crate :: prisma :: rooms :: $ field :: NAME) , + ,] . into_iter () . collect :: < Vec < _ >> () . join (", ")) } fn visit_str < E > (self , value : & str) -> Result < Field , E > where E : :: serde :: de :: Error , { match value { $ (crate :: prisma :: rooms :: $ field :: NAME => Ok (Field :: $ field)) , * , _ => Err (:: serde :: de :: Error :: unknown_field (value , FIELDS)) , } } } deserializer . deserialize_identifier (FieldVisitor) } } struct DataVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for DataVisitor { type Value = Data ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("struct Data") } fn visit_map < V > (self , mut map : V) -> Result < Data , V :: Error > where V : :: serde :: de :: MapAccess < 'de > , { $ (let mut $ field = None ;) * while let Some (key) = map . next_key () ? { match key { $ (Field :: $ field => { if $ field . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: rooms :: $ field :: NAME)) ; } $ field = Some (map . next_value () ?) ; }) * } } $ (let $ field = $ field . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: rooms :: $ field :: NAME)) ? ;) * Ok (Data { $ ($ field) , * }) } } const FIELDS : & 'static [& 'static str] = & ["id" , "name" , "createdAt" , "updatedAt" , "Messages"] ; deserializer . deserialize_struct ("Data" , FIELDS , DataVisitor) } } $ ($ (pub mod $ field { crate :: prisma :: rooms :: $ selection_mode ! (@ field_module ; $ field : $ selection_mode { $ ($ selections) + }) ; }) ?) + } ; (@ field_type ; id) => { i32 } ; (@ field_type ; name) => { String } ; (@ field_type ; created_at) => { :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > } ; (@ field_type ; updated_at) => { :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > } ; (@ field_type ; messages : $ selection_mode : ident { $ ($ selections : tt) + }) => { Vec < messages :: Data > } ; (@ field_type ; messages) => { Vec < crate :: prisma :: messages :: Data > } ; (@ field_type ; $ field : ident $ ($ tokens : tt) *) => { compile_error ! (stringify ! (Cannot include nonexistent relation $ field on model "Rooms" , available relations are "id, name, created_at, updated_at, messages")) } ; (@ field_module ; messages : $ selection_mode : ident { $ ($ selections : tt) + }) => { crate :: prisma :: messages :: select ! (@ definitions ; ; $ ($ selections) +) ; } ; (@ field_module ; $ ($ tokens : tt) *) => { } ; (@ selection_field_to_selection_param ; id) => { Into :: < crate :: prisma :: rooms :: SelectParam > :: into (crate :: prisma :: rooms :: id :: Select) } ; (@ selection_field_to_selection_param ; name) => { Into :: < crate :: prisma :: rooms :: SelectParam > :: into (crate :: prisma :: rooms :: name :: Select) } ; (@ selection_field_to_selection_param ; created_at) => { Into :: < crate :: prisma :: rooms :: SelectParam > :: into (crate :: prisma :: rooms :: created_at :: Select) } ; (@ selection_field_to_selection_param ; updated_at) => { Into :: < crate :: prisma :: rooms :: SelectParam > :: into (crate :: prisma :: rooms :: updated_at :: Select) } ; (@ selection_field_to_selection_param ; messages $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? : $ selection_mode : ident { $ ($ selections : tt) + }) => { { Into :: < crate :: prisma :: rooms :: SelectParam > :: into (crate :: prisma :: rooms :: messages :: Select :: $ selection_mode (crate :: prisma :: messages :: ManyArgs :: new (crate :: prisma :: messages :: select ! (@ filters_to_args ; $ ($ ($ filters) +) ?)) $ ($ (. $ arg ($ ($ arg_params) *)) *) ? , crate :: prisma :: messages :: select ! (@ selections_to_params ; : $ selection_mode { $ ($ selections) + }) . into_iter () . collect ())) } } ; (@ selection_field_to_selection_param ; messages $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ?) => { { Into :: < crate :: prisma :: rooms :: SelectParam > :: into (crate :: prisma :: rooms :: messages :: Select :: Fetch (crate :: prisma :: messages :: ManyArgs :: new (crate :: prisma :: messages :: select ! (@ filters_to_args ; $ ($ ($ filters) +) ?)) $ ($ (. $ arg ($ ($ arg_params) *)) *) ?) ,) } } ; (@ selection_field_to_selection_param ; $ ($ tokens : tt) *) => { compile_error ! (stringify ! ($ ($ tokens) *)) } ; (@ selections_to_params ; : $ macro_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { [$ (crate :: prisma :: rooms :: $ macro_name ! (@ selection_field_to_selection_param ; $ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) ,) +] } ; (@ filters_to_args ;) => { vec ! [] } ; (@ filters_to_args ; $ ($ t : tt) *) => { $ ($ t) * } ; (@ field_serde_name ; id) => { "id" } ; (@ field_serde_name ; name) => { "name" } ; (@ field_serde_name ; created_at) => { "createdAt" } ; (@ field_serde_name ; updated_at) => { "updatedAt" } ; (@ field_serde_name ; messages) => { "Messages" } ; }
    pub use _select_rooms as select;
    pub enum SelectParam {
        Id(id::Select),
        Name(name::Select),
        CreatedAt(created_at::Select),
        UpdatedAt(updated_at::Select),
        Messages(messages::Select),
    }
    impl SelectParam {
        pub fn to_selection(self) -> ::prisma_client_rust::Selection {
            match self {
                Self::Id(data) => data.to_selection(),
                Self::Name(data) => data.to_selection(),
                Self::CreatedAt(data) => data.to_selection(),
                Self::UpdatedAt(data) => data.to_selection(),
                Self::Messages(data) => data.to_selection(),
            }
        }
    }
    #[macro_export]
    macro_rules ! _include_rooms { ($ (($ ($ func_arg : ident : $ func_arg_ty : ty) , +) =>) ? $ module_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { # [allow (warnings)] pub mod $ module_name { crate :: prisma :: rooms :: include ! (@ definitions ; $ module_name ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; use super :: * ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: IncludeType for Selection { type Data = Data ; type ModelData = crate :: prisma :: rooms :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } pub fn include ($ ($ ($ func_arg : $ func_arg_ty) , +) ?) -> Selection { Selection ([crate :: prisma :: rooms :: include ! (@ selections_to_params ; : include { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () , < crate :: prisma :: rooms :: Types as :: prisma_client_rust :: ModelTypes > :: scalar_selections ()] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } } ; ({ $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { { crate :: prisma :: rooms :: include ! (@ definitions ; ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: IncludeType for Selection { type Data = Data ; type ModelData = crate :: prisma :: rooms :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } Selection ([crate :: prisma :: rooms :: include ! (@ selections_to_params ; : include { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () , < crate :: prisma :: rooms :: Types as :: prisma_client_rust :: ModelTypes > :: scalar_selections ()] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } ; (@ definitions ; $ ($ module_name : ident) ? ; $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) +) => { # [allow (warnings)] enum Fields { messages } # [allow (warnings)] impl Fields { fn selections () { $ (let _ = Fields :: $ field ;) + } } # [allow (warnings)] # [derive (std :: fmt :: Debug , Clone)] pub struct Data { pub id : i32 , pub name : String , pub created_at : :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > , pub updated_at : :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > , $ (pub $ field : crate :: prisma :: rooms :: include ! (@ field_type ; $ field $ (: $ selection_mode { $ ($ selections) + }) ?) ,) + } impl :: serde :: Serialize for Data { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : :: serde :: Serializer , { use :: serde :: ser :: SerializeStruct ; let mut state = serializer . serialize_struct ("Data" , [$ (stringify ! ($ field) ,) + stringify ! (id) , stringify ! (name) , stringify ! (created_at) , stringify ! (updated_at)] . len ()) ? ; $ (state . serialize_field (crate :: prisma :: rooms :: $ field :: NAME , & self . $ field) ? ;) * state . serialize_field (crate :: prisma :: rooms :: id :: NAME , & self . id) ? ; state . serialize_field (crate :: prisma :: rooms :: name :: NAME , & self . name) ? ; state . serialize_field (crate :: prisma :: rooms :: created_at :: NAME , & self . created_at) ? ; state . serialize_field (crate :: prisma :: rooms :: updated_at :: NAME , & self . updated_at) ? ; state . end () } } impl < 'de > :: serde :: Deserialize < 'de > for Data { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : :: serde :: Deserializer < 'de > , { # [allow (warnings)] enum Field { $ ($ field) , + , id , name , created_at , updated_at } impl < 'de > :: serde :: Deserialize < 'de > for Field { fn deserialize < D > (deserializer : D) -> Result < Field , D :: Error > where D : :: serde :: Deserializer < 'de > , { struct FieldVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for FieldVisitor { type Value = Field ; fn expecting (& self , formatter : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { formatter . write_str (& [$ (crate :: prisma :: rooms :: $ field :: NAME) , + , crate :: prisma :: rooms :: id :: NAME , crate :: prisma :: rooms :: name :: NAME , crate :: prisma :: rooms :: created_at :: NAME , crate :: prisma :: rooms :: updated_at :: NAME] . into_iter () . collect :: < Vec < _ >> () . join (", ")) } fn visit_str < E > (self , value : & str) -> Result < Field , E > where E : :: serde :: de :: Error , { match value { $ (crate :: prisma :: rooms :: $ field :: NAME => Ok (Field :: $ field)) , * , crate :: prisma :: rooms :: id :: NAME => Ok (Field :: id) , crate :: prisma :: rooms :: name :: NAME => Ok (Field :: name) , crate :: prisma :: rooms :: created_at :: NAME => Ok (Field :: created_at) , crate :: prisma :: rooms :: updated_at :: NAME => Ok (Field :: updated_at) , _ => Err (:: serde :: de :: Error :: unknown_field (value , FIELDS)) , } } } deserializer . deserialize_identifier (FieldVisitor) } } struct DataVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for DataVisitor { type Value = Data ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("struct Data") } fn visit_map < V > (self , mut map : V) -> Result < Data , V :: Error > where V : :: serde :: de :: MapAccess < 'de > , { $ (let mut $ field = None ;) * let mut id = None ; let mut name = None ; let mut created_at = None ; let mut updated_at = None ; while let Some (key) = map . next_key () ? { match key { Field :: id => { if id . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: rooms :: id :: NAME)) ; } id = Some (map . next_value () ?) ; } Field :: name => { if name . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: rooms :: name :: NAME)) ; } name = Some (map . next_value () ?) ; } Field :: created_at => { if created_at . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: rooms :: created_at :: NAME)) ; } created_at = Some (map . next_value () ?) ; } Field :: updated_at => { if updated_at . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: rooms :: updated_at :: NAME)) ; } updated_at = Some (map . next_value () ?) ; } $ (Field :: $ field => { if $ field . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: rooms :: $ field :: NAME)) ; } $ field = Some (map . next_value () ?) ; }) * } } $ (let $ field = $ field . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: rooms :: $ field :: NAME)) ? ;) * let id = id . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: rooms :: id :: NAME)) ? ; let name = name . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: rooms :: name :: NAME)) ? ; let created_at = created_at . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: rooms :: created_at :: NAME)) ? ; let updated_at = updated_at . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: rooms :: updated_at :: NAME)) ? ; Ok (Data { id , name , created_at , updated_at , $ ($ field) , * }) } } const FIELDS : & 'static [& 'static str] = & ["id" , "name" , "createdAt" , "updatedAt" , "Messages"] ; deserializer . deserialize_struct ("Data" , FIELDS , DataVisitor) } } $ ($ (pub mod $ field { crate :: prisma :: rooms :: $ selection_mode ! (@ field_module ; $ field : $ selection_mode { $ ($ selections) + }) ; }) ?) + } ; (@ field_type ; messages : $ selection_mode : ident { $ ($ selections : tt) + }) => { Vec < messages :: Data > } ; (@ field_type ; messages) => { Vec < crate :: prisma :: messages :: Data > } ; (@ field_type ; $ field : ident $ ($ tokens : tt) *) => { compile_error ! (stringify ! (Cannot include nonexistent relation $ field on model "Rooms" , available relations are "messages")) } ; (@ field_module ; messages : $ selection_mode : ident { $ ($ selections : tt) + }) => { crate :: prisma :: messages :: include ! (@ definitions ; ; $ ($ selections) +) ; } ; (@ field_module ; $ ($ tokens : tt) *) => { } ; (@ selection_field_to_selection_param ; messages $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? : $ selection_mode : ident { $ ($ selections : tt) + }) => { { Into :: < crate :: prisma :: rooms :: IncludeParam > :: into (crate :: prisma :: rooms :: messages :: Include :: $ selection_mode (crate :: prisma :: messages :: ManyArgs :: new (crate :: prisma :: messages :: include ! (@ filters_to_args ; $ ($ ($ filters) +) ?)) $ ($ (. $ arg ($ ($ arg_params) *)) *) ? , crate :: prisma :: messages :: select ! (@ selections_to_params ; : $ selection_mode { $ ($ selections) + }) . into_iter () . collect ())) } } ; (@ selection_field_to_selection_param ; messages $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ?) => { { Into :: < crate :: prisma :: rooms :: IncludeParam > :: into (crate :: prisma :: rooms :: messages :: Include :: Fetch (crate :: prisma :: messages :: ManyArgs :: new (crate :: prisma :: messages :: include ! (@ filters_to_args ; $ ($ ($ filters) +) ?)) $ ($ (. $ arg ($ ($ arg_params) *)) *) ?) ,) } } ; (@ selection_field_to_selection_param ; $ ($ tokens : tt) *) => { compile_error ! (stringify ! ($ ($ tokens) *)) } ; (@ selections_to_params ; : $ macro_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { [$ (crate :: prisma :: rooms :: $ macro_name ! (@ selection_field_to_selection_param ; $ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) ,) +] } ; (@ filters_to_args ;) => { vec ! [] } ; (@ filters_to_args ; $ ($ t : tt) *) => { $ ($ t) * } ; (@ field_serde_name ; id) => { "id" } ; (@ field_serde_name ; name) => { "name" } ; (@ field_serde_name ; created_at) => { "createdAt" } ; (@ field_serde_name ; updated_at) => { "updatedAt" } ; (@ field_serde_name ; messages) => { "Messages" } ; }
    pub use _include_rooms as include;
    pub enum IncludeParam {
        Id(id::Include),
        Name(name::Include),
        CreatedAt(created_at::Include),
        UpdatedAt(updated_at::Include),
        Messages(messages::Include),
    }
    impl IncludeParam {
        pub fn to_selection(self) -> ::prisma_client_rust::Selection {
            match self {
                Self::Id(data) => data.to_selection(),
                Self::Name(data) => data.to_selection(),
                Self::CreatedAt(data) => data.to_selection(),
                Self::UpdatedAt(data) => data.to_selection(),
                Self::Messages(data) => data.to_selection(),
            }
        }
    }
    #[macro_export]
    macro_rules ! _partial_unchecked_rooms { ($ struct_name : ident { $ ($ scalar_field : ident) + }) => { :: prisma_client_rust :: macros :: partial_unchecked ! { crate :: prisma :: rooms struct $ struct_name { # [serde (rename = "id")] pub id : i32 , # [serde (rename = "name")] pub name : String , # [serde (rename = "createdAt")] pub created_at : :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > , # [serde (rename = "updatedAt")] pub updated_at : :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > } [$ ($ scalar_field) , +] } } ; }
    pub use _partial_unchecked_rooms as partial_unchecked;
    #[derive(Debug, Clone, :: serde :: Serialize, :: serde :: Deserialize)]
    pub struct Data {
        #[serde(rename = "id")]
        pub id: i32,
        #[serde(rename = "name")]
        pub name: String,
        #[serde(rename = "createdAt")]
        pub created_at:
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        #[serde(rename = "updatedAt")]
        pub updated_at:
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        #[serde(rename = "Messages")]
        pub messages: Option<Vec<super::messages::Data>>,
    }
    impl Data {
        pub fn messages(
            &self,
        ) -> Result<&Vec<super::messages::Data>, ::prisma_client_rust::RelationNotFetchedError>
        {
            self.messages
                .as_ref()
                .ok_or(::prisma_client_rust::RelationNotFetchedError::new(
                    stringify!(messages),
                ))
        }
    }
    #[derive(Clone)]
    pub enum WithParam {
        Messages(super::messages::ManyArgs),
    }
    impl Into<::prisma_client_rust::Selection> for WithParam {
        fn into(self) -> ::prisma_client_rust::Selection {
            match self {
                Self::Messages(args) => {
                    let (arguments, mut nested_selections) = args.to_graphql();
                    nested_selections . extend (< super :: messages :: Types as :: prisma_client_rust :: ModelTypes > :: scalar_selections ()) ;
                    ::prisma_client_rust::Selection::new(
                        messages::NAME,
                        None,
                        arguments,
                        nested_selections,
                    )
                }
            }
        }
    }
    #[derive(Clone)]
    pub enum SetParam {
        SetId(i32),
        IncrementId(i32),
        DecrementId(i32),
        MultiplyId(i32),
        DivideId(i32),
        SetName(String),
        SetCreatedAt(
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        ),
        SetUpdatedAt(
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        ),
        ConnectMessages(Vec<super::messages::UniqueWhereParam>),
        DisconnectMessages(Vec<super::messages::UniqueWhereParam>),
        SetMessages(Vec<super::messages::UniqueWhereParam>),
    }
    impl From<SetParam> for (String, ::prisma_client_rust::PrismaValue) {
        fn from(param: SetParam) -> Self {
            match param {
                SetParam::SetId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Int(value as i64),
                ),
                SetParam::IncrementId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "increment".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::DecrementId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "decrement".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::MultiplyId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "multiply".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::DivideId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "divide".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::SetName(value) => (
                    name::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(value),
                ),
                SetParam::SetCreatedAt(value) => (
                    created_at::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::DateTime(value),
                ),
                SetParam::SetUpdatedAt(value) => (
                    updated_at::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::DateTime(value),
                ),
                SetParam::ConnectMessages(where_params) => (
                    messages::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "connect".to_string(),
                        ::prisma_client_rust::PrismaValue::List(
                            where_params
                                .into_iter()
                                .map(Into::<super::messages::WhereParam>::into)
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .map(|v| ::prisma_client_rust::PrismaValue::Object(vec![v]))
                                .collect(),
                        ),
                    )]),
                ),
                SetParam::DisconnectMessages(where_params) => (
                    messages::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "disconnect".to_string(),
                        ::prisma_client_rust::PrismaValue::List(
                            where_params
                                .into_iter()
                                .map(Into::<super::messages::WhereParam>::into)
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .map(|v| ::prisma_client_rust::PrismaValue::Object(vec![v]))
                                .collect(),
                        ),
                    )]),
                ),
                SetParam::SetMessages(where_params) => (
                    messages::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "set".to_string(),
                        ::prisma_client_rust::PrismaValue::List(
                            where_params
                                .into_iter()
                                .map(Into::<super::messages::WhereParam>::into)
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .map(|v| ::prisma_client_rust::PrismaValue::Object(vec![v]))
                                .collect(),
                        ),
                    )]),
                ),
            }
        }
    }
    #[derive(Clone)]
    pub enum UncheckedSetParam {
        Id(i32),
        Name(String),
        CreatedAt(
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        ),
        UpdatedAt(
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        ),
    }
    impl From<UncheckedSetParam> for SetParam {
        fn from(param: UncheckedSetParam) -> Self {
            match param {
                UncheckedSetParam::Id(value) => Self::SetId(value),
                UncheckedSetParam::Name(value) => Self::SetName(value),
                UncheckedSetParam::CreatedAt(value) => Self::SetCreatedAt(value),
                UncheckedSetParam::UpdatedAt(value) => Self::SetUpdatedAt(value),
            }
        }
    }
    #[derive(Clone)]
    pub enum OrderByParam {
        Id(::prisma_client_rust::Direction),
        Name(::prisma_client_rust::Direction),
        CreatedAt(::prisma_client_rust::Direction),
        UpdatedAt(::prisma_client_rust::Direction),
    }
    impl Into<(String, ::prisma_client_rust::PrismaValue)> for OrderByParam {
        fn into(self) -> (String, ::prisma_client_rust::PrismaValue) {
            match self {
                Self::Id(direction) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
                Self::Name(direction) => (
                    name::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
                Self::CreatedAt(direction) => (
                    created_at::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
                Self::UpdatedAt(direction) => (
                    updated_at::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
            }
        }
    }
    #[derive(Clone)]
    pub enum WhereParam {
        Not(Vec<WhereParam>),
        Or(Vec<WhereParam>),
        And(Vec<WhereParam>),
        Id(_prisma::read_filters::IntFilter),
        Name(_prisma::read_filters::StringFilter),
        CreatedAt(_prisma::read_filters::DateTimeFilter),
        UpdatedAt(_prisma::read_filters::DateTimeFilter),
        MessagesSome(Vec<super::messages::WhereParam>),
        MessagesEvery(Vec<super::messages::WhereParam>),
        MessagesNone(Vec<super::messages::WhereParam>),
    }
    impl ::prisma_client_rust::WhereInput for WhereParam {
        fn serialize(self) -> ::prisma_client_rust::SerializedWhereInput {
            let (name, value) = match self {
                Self::Not(value) => (
                    "NOT",
                    ::prisma_client_rust::SerializedWhereValue::Object(
                        ::prisma_client_rust::merge_fields(
                            value
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(Into::into)
                                .collect(),
                        ),
                    ),
                ),
                Self::Or(value) => (
                    "OR",
                    ::prisma_client_rust::SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(::prisma_client_rust::WhereInput::serialize)
                            .map(Into::into)
                            .map(|v| vec![v])
                            .map(::prisma_client_rust::PrismaValue::Object)
                            .collect(),
                    ),
                ),
                Self::And(value) => (
                    "AND",
                    ::prisma_client_rust::SerializedWhereValue::Object(
                        ::prisma_client_rust::merge_fields(
                            value
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(Into::into)
                                .collect(),
                        ),
                    ),
                ),
                Self::Id(value) => (id::NAME, value.into()),
                Self::Name(value) => (name::NAME, value.into()),
                Self::CreatedAt(value) => (created_at::NAME, value.into()),
                Self::UpdatedAt(value) => (updated_at::NAME, value.into()),
                Self::MessagesSome(where_params) => (
                    messages::NAME,
                    ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "some".to_string(),
                        ::prisma_client_rust::PrismaValue::Object(
                            where_params
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .collect(),
                        ),
                    )]),
                ),
                Self::MessagesEvery(where_params) => (
                    messages::NAME,
                    ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "every".to_string(),
                        ::prisma_client_rust::PrismaValue::Object(
                            where_params
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .collect(),
                        ),
                    )]),
                ),
                Self::MessagesNone(where_params) => (
                    messages::NAME,
                    ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "none".to_string(),
                        ::prisma_client_rust::PrismaValue::Object(
                            where_params
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .collect(),
                        ),
                    )]),
                ),
            };
            ::prisma_client_rust::SerializedWhereInput::new(name, value.into())
        }
    }
    #[derive(Clone)]
    pub enum UniqueWhereParam {
        NameEquals(String),
        IdEquals(i32),
    }
    impl From<UniqueWhereParam> for WhereParam {
        fn from(value: UniqueWhereParam) -> Self {
            match value {
                UniqueWhereParam::NameEquals(value) => {
                    Self::Name(_prisma::read_filters::StringFilter::Equals(value))
                }
                UniqueWhereParam::IdEquals(value) => {
                    Self::Id(_prisma::read_filters::IntFilter::Equals(value))
                }
            }
        }
    }
    impl From<::prisma_client_rust::Operator<Self>> for WhereParam {
        fn from(op: ::prisma_client_rust::Operator<Self>) -> Self {
            match op {
                ::prisma_client_rust::Operator::Not(value) => Self::Not(value),
                ::prisma_client_rust::Operator::And(value) => Self::And(value),
                ::prisma_client_rust::Operator::Or(value) => Self::Or(value),
            }
        }
    }
    #[derive(Clone)]
    pub struct Types;
    impl ::prisma_client_rust::ModelTypes for Types {
        type Data = Data;
        type Where = WhereParam;
        type UncheckedSet = UncheckedSetParam;
        type Set = SetParam;
        type With = WithParam;
        type OrderBy = OrderByParam;
        type Cursor = UniqueWhereParam;
        const MODEL: &'static str = NAME;
        fn scalar_selections() -> Vec<::prisma_client_rust::Selection> {
            vec![
                ::prisma_client_rust::sel(id::NAME),
                ::prisma_client_rust::sel(name::NAME),
                ::prisma_client_rust::sel(created_at::NAME),
                ::prisma_client_rust::sel(updated_at::NAME),
            ]
        }
    }
    pub type UniqueArgs = ::prisma_client_rust::UniqueArgs<Types>;
    pub type ManyArgs = ::prisma_client_rust::ManyArgs<Types>;
    pub type Count<'a> = ::prisma_client_rust::Count<'a, Types>;
    pub type Create<'a> = ::prisma_client_rust::Create<'a, Types>;
    pub type CreateMany<'a> = ::prisma_client_rust::CreateMany<'a, Types>;
    pub type FindUnique<'a> = ::prisma_client_rust::FindUnique<'a, Types>;
    pub type FindMany<'a> = ::prisma_client_rust::FindMany<'a, Types>;
    pub type FindFirst<'a> = ::prisma_client_rust::FindFirst<'a, Types>;
    pub type Update<'a> = ::prisma_client_rust::Update<'a, Types>;
    pub type UpdateMany<'a> = ::prisma_client_rust::UpdateMany<'a, Types>;
    pub type Upsert<'a> = ::prisma_client_rust::Upsert<'a, Types>;
    pub type Delete<'a> = ::prisma_client_rust::Delete<'a, Types>;
    pub type DeleteMany<'a> = ::prisma_client_rust::DeleteMany<'a, Types>;
    #[derive(Clone)]
    pub struct Actions<'a> {
        pub client: &'a ::prisma_client_rust::PrismaClientInternals,
    }
    impl<'a> Actions<'a> {
        pub fn find_unique(self, _where: UniqueWhereParam) -> FindUnique<'a> {
            FindUnique::new(self.client, _where.into())
        }
        pub fn find_first(self, _where: Vec<WhereParam>) -> FindFirst<'a> {
            FindFirst::new(self.client, _where)
        }
        pub fn find_many(self, _where: Vec<WhereParam>) -> FindMany<'a> {
            FindMany::new(self.client, _where)
        }
        pub fn create(self, name: String, mut _params: Vec<SetParam>) -> Create<'a> {
            _params.extend([name::set(name)]);
            Create::new(self.client, _params)
        }
        pub fn create_unchecked(
            self,
            name: String,
            mut _params: Vec<UncheckedSetParam>,
        ) -> Create<'a> {
            _params.extend([name::set(name)]);
            Create::new(self.client, _params.into_iter().map(Into::into).collect())
        }
        pub fn create_many(self, data: Vec<(String, Vec<SetParam>)>) -> CreateMany<'a> {
            let data = data
                .into_iter()
                .map(|(name, mut _params)| {
                    _params.extend([name::set(name)]);
                    _params
                })
                .collect();
            CreateMany::new(self.client, data)
        }
        pub fn update(self, _where: UniqueWhereParam, _params: Vec<SetParam>) -> Update<'a> {
            Update::new(self.client, _where.into(), _params, vec![])
        }
        pub fn update_unchecked(
            self,
            _where: UniqueWhereParam,
            _params: Vec<UncheckedSetParam>,
        ) -> Update<'a> {
            Update::new(
                self.client,
                _where.into(),
                _params.into_iter().map(Into::into).collect(),
                vec![],
            )
        }
        pub fn update_many(
            self,
            _where: Vec<WhereParam>,
            _params: Vec<SetParam>,
        ) -> UpdateMany<'a> {
            UpdateMany::new(self.client, _where, _params)
        }
        pub fn upsert(
            self,
            _where: UniqueWhereParam,
            (name, mut _params): (String, Vec<SetParam>),
            _update: Vec<SetParam>,
        ) -> Upsert<'a> {
            _params.extend([name::set(name)]);
            Upsert::new(self.client, _where.into(), _params, _update)
        }
        pub fn delete(self, _where: UniqueWhereParam) -> Delete<'a> {
            Delete::new(self.client, _where.into(), vec![])
        }
        pub fn delete_many(self, _where: Vec<WhereParam>) -> DeleteMany<'a> {
            DeleteMany::new(self.client, _where)
        }
        pub fn count(self, _where: Vec<WhereParam>) -> Count<'a> {
            Count::new(self.client, _where)
        }
        pub fn find_raw<T: ::prisma_client_rust::Data>(
            self,
        ) -> ::prisma_client_rust::FindRaw<'a, Types, T> {
            ::prisma_client_rust::FindRaw::new(self.client)
        }
        pub fn aggregate_raw<T: ::prisma_client_rust::Data>(
            self,
        ) -> ::prisma_client_rust::AggregateRaw<'a, Types, T> {
            ::prisma_client_rust::AggregateRaw::new(self.client)
        }
    }
}
pub mod messages {
    use super::_prisma::*;
    use super::*;
    pub const NAME: &str = "Messages";
    pub mod id {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "id";
        pub struct Set(pub i32);
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetId(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::Id(v)
            }
        }
        pub fn set<T: From<Set>>(value: i32) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::Id(direction)
        }
        pub fn equals<T: From<UniqueWhereParam>>(value: i32) -> T {
            UniqueWhereParam::IdEquals(value).into()
        }
        ::prisma_client_rust::scalar_where_param_fns!(_prisma::read_filters::IntFilter, Id, {
            fn in_vec(_: Vec<i32>) -> InVec;
            fn not_in_vec(_: Vec<i32>) -> NotInVec;
            fn lt(_: i32) -> Lt;
            fn lte(_: i32) -> Lte;
            fn gt(_: i32) -> Gt;
            fn gte(_: i32) -> Gte;
            fn not(_: i32) -> Not;
        });
        pub fn increment(value: i32) -> SetParam {
            SetParam::IncrementId(value)
        }
        pub fn decrement(value: i32) -> SetParam {
            SetParam::DecrementId(value)
        }
        pub fn multiply(value: i32) -> SetParam {
            SetParam::MultiplyId(value)
        }
        pub fn divide(value: i32) -> SetParam {
            SetParam::DivideId(value)
        }
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::Id(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::Id(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod message {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "message";
        pub struct Set(pub String);
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetMessage(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::Message(v)
            }
        }
        pub fn set<T: From<Set>>(value: String) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::Message(direction)
        }
        pub fn equals(value: String) -> WhereParam {
            WhereParam::Message(_prisma::read_filters::StringFilter::Equals(value))
        }
        ::prisma_client_rust::scalar_where_param_fns!(
            _prisma::read_filters::StringFilter,
            Message,
            {
                fn in_vec(_: Vec<String>) -> InVec;
                fn not_in_vec(_: Vec<String>) -> NotInVec;
                fn lt(_: String) -> Lt;
                fn lte(_: String) -> Lte;
                fn gt(_: String) -> Gt;
                fn gte(_: String) -> Gte;
                fn contains(_: String) -> Contains;
                fn starts_with(_: String) -> StartsWith;
                fn ends_with(_: String) -> EndsWith;
                fn not(_: String) -> Not;
            }
        );
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::Message(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::Message(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod created_at {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "createdAt";
        pub struct Set(
            pub ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        );
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetCreatedAt(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::CreatedAt(v)
            }
        }
        pub fn set<T: From<Set>>(
            value: ::prisma_client_rust::chrono::DateTime<
                ::prisma_client_rust::chrono::FixedOffset,
            >,
        ) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::CreatedAt(direction)
        }
        pub fn equals(
            value: ::prisma_client_rust::chrono::DateTime<
                ::prisma_client_rust::chrono::FixedOffset,
            >,
        ) -> WhereParam {
            WhereParam::CreatedAt(_prisma::read_filters::DateTimeFilter::Equals(value))
        }
        ::prisma_client_rust::scalar_where_param_fns!(
            _prisma::read_filters::DateTimeFilter,
            CreatedAt,
            {
                fn in_vec(
                    _: Vec<
                        ::prisma_client_rust::chrono::DateTime<
                            ::prisma_client_rust::chrono::FixedOffset,
                        >,
                    >,
                ) -> InVec;
                fn not_in_vec(
                    _: Vec<
                        ::prisma_client_rust::chrono::DateTime<
                            ::prisma_client_rust::chrono::FixedOffset,
                        >,
                    >,
                ) -> NotInVec;
                fn lt(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Lt;
                fn lte(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Lte;
                fn gt(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Gt;
                fn gte(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Gte;
                fn not(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Not;
            }
        );
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::CreatedAt(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::CreatedAt(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod updated_at {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "updatedAt";
        pub struct Set(
            pub ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        );
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetUpdatedAt(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::UpdatedAt(v)
            }
        }
        pub fn set<T: From<Set>>(
            value: ::prisma_client_rust::chrono::DateTime<
                ::prisma_client_rust::chrono::FixedOffset,
            >,
        ) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::UpdatedAt(direction)
        }
        pub fn equals(
            value: ::prisma_client_rust::chrono::DateTime<
                ::prisma_client_rust::chrono::FixedOffset,
            >,
        ) -> WhereParam {
            WhereParam::UpdatedAt(_prisma::read_filters::DateTimeFilter::Equals(value))
        }
        ::prisma_client_rust::scalar_where_param_fns!(
            _prisma::read_filters::DateTimeFilter,
            UpdatedAt,
            {
                fn in_vec(
                    _: Vec<
                        ::prisma_client_rust::chrono::DateTime<
                            ::prisma_client_rust::chrono::FixedOffset,
                        >,
                    >,
                ) -> InVec;
                fn not_in_vec(
                    _: Vec<
                        ::prisma_client_rust::chrono::DateTime<
                            ::prisma_client_rust::chrono::FixedOffset,
                        >,
                    >,
                ) -> NotInVec;
                fn lt(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Lt;
                fn lte(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Lte;
                fn gt(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Gt;
                fn gte(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Gte;
                fn not(
                    _: ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                ) -> Not;
            }
        );
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::UpdatedAt(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::UpdatedAt(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod user {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "user";
        pub struct Fetch(pub user::UniqueArgs);
        impl Fetch {
            pub fn with(mut self, params: impl Into<user::WithParam>) -> Self {
                self.0 = self.0.with(params.into());
                self
            }
        }
        impl From<Fetch> for WithParam {
            fn from(Fetch(v): Fetch) -> Self {
                WithParam::User(v)
            }
        }
        pub fn fetch() -> Fetch {
            Fetch(user::UniqueArgs::new())
        }
        pub struct Connect(user::UniqueWhereParam);
        impl From<Connect> for SetParam {
            fn from(Connect(v): Connect) -> Self {
                Self::ConnectUser(v)
            }
        }
        pub fn connect<T: From<Connect>>(value: user::UniqueWhereParam) -> T {
            Connect(value).into()
        }
        pub fn is(value: Vec<user::WhereParam>) -> WhereParam {
            WhereParam::UserIs(value)
        }
        pub fn is_not(value: Vec<user::WhereParam>) -> WhereParam {
            WhereParam::UserIsNot(value)
        }
        pub enum Include {
            Select(Vec<user::SelectParam>),
            Include(Vec<user::IncludeParam>),
            Fetch,
        }
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::User(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                let selections = match self {
                    Self::Select(selections) => {
                        selections.into_iter().map(|s| s.to_selection()).collect()
                    }
                    Self::Include(selections) => {
                        let mut nested_selections =
                            <user::Types as ::prisma_client_rust::ModelTypes>::scalar_selections();
                        nested_selections.extend(selections.into_iter().map(|s| s.to_selection()));
                        nested_selections
                    }
                    Self::Fetch => {
                        <user::Types as ::prisma_client_rust::ModelTypes>::scalar_selections()
                    }
                };
                ::prisma_client_rust::Selection::new("user", None, [], selections)
            }
            pub fn select(nested_selections: Vec<user::SelectParam>) -> Self {
                Self::Select(nested_selections)
            }
            pub fn include(nested_selections: Vec<user::IncludeParam>) -> Self {
                Self::Include(nested_selections)
            }
        }
        pub enum Select {
            Select(Vec<user::SelectParam>),
            Include(Vec<user::IncludeParam>),
            Fetch,
        }
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::User(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                let selections = match self {
                    Self::Select(selections) => {
                        selections.into_iter().map(|s| s.to_selection()).collect()
                    }
                    Self::Include(selections) => {
                        let mut nested_selections = vec![];
                        nested_selections.extend(selections.into_iter().map(|s| s.to_selection()));
                        nested_selections
                    }
                    Self::Fetch => {
                        <user::Types as ::prisma_client_rust::ModelTypes>::scalar_selections()
                    }
                };
                ::prisma_client_rust::Selection::new("user", None, [], selections)
            }
            pub fn select(nested_selections: Vec<user::SelectParam>) -> Self {
                Self::Select(nested_selections)
            }
            pub fn include(nested_selections: Vec<user::IncludeParam>) -> Self {
                Self::Include(nested_selections)
            }
        }
    }
    pub mod user_id {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "userId";
        pub struct Set(pub i32);
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetUserId(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::UserId(v)
            }
        }
        pub fn set<T: From<Set>>(value: i32) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::UserId(direction)
        }
        pub fn equals(value: i32) -> WhereParam {
            WhereParam::UserId(_prisma::read_filters::IntFilter::Equals(value))
        }
        ::prisma_client_rust::scalar_where_param_fns!(_prisma::read_filters::IntFilter, UserId, {
            fn in_vec(_: Vec<i32>) -> InVec;
            fn not_in_vec(_: Vec<i32>) -> NotInVec;
            fn lt(_: i32) -> Lt;
            fn lte(_: i32) -> Lte;
            fn gt(_: i32) -> Gt;
            fn gte(_: i32) -> Gte;
            fn not(_: i32) -> Not;
        });
        pub fn increment(value: i32) -> SetParam {
            SetParam::IncrementUserId(value)
        }
        pub fn decrement(value: i32) -> SetParam {
            SetParam::DecrementUserId(value)
        }
        pub fn multiply(value: i32) -> SetParam {
            SetParam::MultiplyUserId(value)
        }
        pub fn divide(value: i32) -> SetParam {
            SetParam::DivideUserId(value)
        }
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::UserId(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::UserId(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub mod room {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "room";
        pub struct Fetch(pub rooms::UniqueArgs);
        impl Fetch {
            pub fn with(mut self, params: impl Into<rooms::WithParam>) -> Self {
                self.0 = self.0.with(params.into());
                self
            }
        }
        impl From<Fetch> for WithParam {
            fn from(Fetch(v): Fetch) -> Self {
                WithParam::Room(v)
            }
        }
        pub fn fetch() -> Fetch {
            Fetch(rooms::UniqueArgs::new())
        }
        pub struct Connect(rooms::UniqueWhereParam);
        impl From<Connect> for SetParam {
            fn from(Connect(v): Connect) -> Self {
                Self::ConnectRoom(v)
            }
        }
        pub fn connect<T: From<Connect>>(value: rooms::UniqueWhereParam) -> T {
            Connect(value).into()
        }
        pub fn is(value: Vec<rooms::WhereParam>) -> WhereParam {
            WhereParam::RoomIs(value)
        }
        pub fn is_not(value: Vec<rooms::WhereParam>) -> WhereParam {
            WhereParam::RoomIsNot(value)
        }
        pub enum Include {
            Select(Vec<rooms::SelectParam>),
            Include(Vec<rooms::IncludeParam>),
            Fetch,
        }
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::Room(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                let selections = match self {
                    Self::Select(selections) => {
                        selections.into_iter().map(|s| s.to_selection()).collect()
                    }
                    Self::Include(selections) => {
                        let mut nested_selections =
                            <rooms::Types as ::prisma_client_rust::ModelTypes>::scalar_selections();
                        nested_selections.extend(selections.into_iter().map(|s| s.to_selection()));
                        nested_selections
                    }
                    Self::Fetch => {
                        <rooms::Types as ::prisma_client_rust::ModelTypes>::scalar_selections()
                    }
                };
                ::prisma_client_rust::Selection::new("room", None, [], selections)
            }
            pub fn select(nested_selections: Vec<rooms::SelectParam>) -> Self {
                Self::Select(nested_selections)
            }
            pub fn include(nested_selections: Vec<rooms::IncludeParam>) -> Self {
                Self::Include(nested_selections)
            }
        }
        pub enum Select {
            Select(Vec<rooms::SelectParam>),
            Include(Vec<rooms::IncludeParam>),
            Fetch,
        }
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::Room(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                let selections = match self {
                    Self::Select(selections) => {
                        selections.into_iter().map(|s| s.to_selection()).collect()
                    }
                    Self::Include(selections) => {
                        let mut nested_selections = vec![];
                        nested_selections.extend(selections.into_iter().map(|s| s.to_selection()));
                        nested_selections
                    }
                    Self::Fetch => {
                        <rooms::Types as ::prisma_client_rust::ModelTypes>::scalar_selections()
                    }
                };
                ::prisma_client_rust::Selection::new("room", None, [], selections)
            }
            pub fn select(nested_selections: Vec<rooms::SelectParam>) -> Self {
                Self::Select(nested_selections)
            }
            pub fn include(nested_selections: Vec<rooms::IncludeParam>) -> Self {
                Self::Include(nested_selections)
            }
        }
    }
    pub mod room_id {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "roomId";
        pub struct Set(pub i32);
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetRoomId(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::RoomId(v)
            }
        }
        pub fn set<T: From<Set>>(value: i32) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::RoomId(direction)
        }
        pub fn equals(value: i32) -> WhereParam {
            WhereParam::RoomId(_prisma::read_filters::IntFilter::Equals(value))
        }
        ::prisma_client_rust::scalar_where_param_fns!(_prisma::read_filters::IntFilter, RoomId, {
            fn in_vec(_: Vec<i32>) -> InVec;
            fn not_in_vec(_: Vec<i32>) -> NotInVec;
            fn lt(_: i32) -> Lt;
            fn lte(_: i32) -> Lte;
            fn gt(_: i32) -> Gt;
            fn gte(_: i32) -> Gte;
            fn not(_: i32) -> Not;
        });
        pub fn increment(value: i32) -> SetParam {
            SetParam::IncrementRoomId(value)
        }
        pub fn decrement(value: i32) -> SetParam {
            SetParam::DecrementRoomId(value)
        }
        pub fn multiply(value: i32) -> SetParam {
            SetParam::MultiplyRoomId(value)
        }
        pub fn divide(value: i32) -> SetParam {
            SetParam::DivideRoomId(value)
        }
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::RoomId(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::RoomId(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub fn create(
        message: String,
        user: super::user::UniqueWhereParam,
        room: super::rooms::UniqueWhereParam,
        _params: Vec<SetParam>,
    ) -> (
        String,
        super::user::UniqueWhereParam,
        super::rooms::UniqueWhereParam,
        Vec<SetParam>,
    ) {
        (message, user, room, _params)
    }
    pub fn create_unchecked(
        message: String,
        user_id: i32,
        room_id: i32,
        _params: Vec<SetParam>,
    ) -> (String, i32, i32, Vec<SetParam>) {
        (message, user_id, room_id, _params)
    }
    #[macro_export]
    macro_rules ! _select_messages { ($ (($ ($ func_arg : ident : $ func_arg_ty : ty) , +) =>) ? $ module_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { # [allow (warnings)] pub mod $ module_name { crate :: prisma :: messages :: select ! (@ definitions ; $ module_name ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; use super :: * ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: SelectType for Selection { type Data = Data ; type ModelData = crate :: prisma :: messages :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } pub fn select ($ ($ ($ func_arg : $ func_arg_ty) , +) ?) -> Selection { Selection ([crate :: prisma :: messages :: select ! (@ selections_to_params ; : select { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () ,] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } } ; ({ $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { { crate :: prisma :: messages :: select ! (@ definitions ; ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: SelectType for Selection { type Data = Data ; type ModelData = crate :: prisma :: messages :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } Selection ([crate :: prisma :: messages :: select ! (@ selections_to_params ; : select { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () ,] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } ; (@ definitions ; $ ($ module_name : ident) ? ; $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) +) => { # [allow (warnings)] enum Fields { id , message , created_at , updated_at , user , user_id , room , room_id } # [allow (warnings)] impl Fields { fn selections () { $ (let _ = Fields :: $ field ;) + } } # [allow (warnings)] # [derive (std :: fmt :: Debug , Clone)] pub struct Data { $ (pub $ field : crate :: prisma :: messages :: select ! (@ field_type ; $ field $ (: $ selection_mode { $ ($ selections) + }) ?) ,) + } impl :: serde :: Serialize for Data { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : :: serde :: Serializer , { use :: serde :: ser :: SerializeStruct ; let mut state = serializer . serialize_struct ("Data" , [$ (stringify ! ($ field) ,) +] . len ()) ? ; $ (state . serialize_field (crate :: prisma :: messages :: $ field :: NAME , & self . $ field) ? ;) * state . end () } } impl < 'de > :: serde :: Deserialize < 'de > for Data { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : :: serde :: Deserializer < 'de > , { # [allow (warnings)] enum Field { $ ($ field) , + , } impl < 'de > :: serde :: Deserialize < 'de > for Field { fn deserialize < D > (deserializer : D) -> Result < Field , D :: Error > where D : :: serde :: Deserializer < 'de > , { struct FieldVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for FieldVisitor { type Value = Field ; fn expecting (& self , formatter : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { formatter . write_str (& [$ (crate :: prisma :: messages :: $ field :: NAME) , + ,] . into_iter () . collect :: < Vec < _ >> () . join (", ")) } fn visit_str < E > (self , value : & str) -> Result < Field , E > where E : :: serde :: de :: Error , { match value { $ (crate :: prisma :: messages :: $ field :: NAME => Ok (Field :: $ field)) , * , _ => Err (:: serde :: de :: Error :: unknown_field (value , FIELDS)) , } } } deserializer . deserialize_identifier (FieldVisitor) } } struct DataVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for DataVisitor { type Value = Data ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("struct Data") } fn visit_map < V > (self , mut map : V) -> Result < Data , V :: Error > where V : :: serde :: de :: MapAccess < 'de > , { $ (let mut $ field = None ;) * while let Some (key) = map . next_key () ? { match key { $ (Field :: $ field => { if $ field . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: messages :: $ field :: NAME)) ; } $ field = Some (map . next_value () ?) ; }) * } } $ (let $ field = $ field . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: messages :: $ field :: NAME)) ? ;) * Ok (Data { $ ($ field) , * }) } } const FIELDS : & 'static [& 'static str] = & ["id" , "message" , "createdAt" , "updatedAt" , "user" , "userId" , "room" , "roomId"] ; deserializer . deserialize_struct ("Data" , FIELDS , DataVisitor) } } $ ($ (pub mod $ field { crate :: prisma :: messages :: $ selection_mode ! (@ field_module ; $ field : $ selection_mode { $ ($ selections) + }) ; }) ?) + } ; (@ field_type ; id) => { i32 } ; (@ field_type ; message) => { String } ; (@ field_type ; created_at) => { :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > } ; (@ field_type ; updated_at) => { :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > } ; (@ field_type ; user : $ selection_mode : ident { $ ($ selections : tt) + }) => { user :: Data } ; (@ field_type ; user) => { crate :: prisma :: user :: Data } ; (@ field_type ; user_id) => { i32 } ; (@ field_type ; room : $ selection_mode : ident { $ ($ selections : tt) + }) => { room :: Data } ; (@ field_type ; room) => { crate :: prisma :: rooms :: Data } ; (@ field_type ; room_id) => { i32 } ; (@ field_type ; $ field : ident $ ($ tokens : tt) *) => { compile_error ! (stringify ! (Cannot include nonexistent relation $ field on model "Messages" , available relations are "id, message, created_at, updated_at, user, user_id, room, room_id")) } ; (@ field_module ; user : $ selection_mode : ident { $ ($ selections : tt) + }) => { crate :: prisma :: user :: select ! (@ definitions ; ; $ ($ selections) +) ; } ; (@ field_module ; room : $ selection_mode : ident { $ ($ selections : tt) + }) => { crate :: prisma :: rooms :: select ! (@ definitions ; ; $ ($ selections) +) ; } ; (@ field_module ; $ ($ tokens : tt) *) => { } ; (@ selection_field_to_selection_param ; id) => { Into :: < crate :: prisma :: messages :: SelectParam > :: into (crate :: prisma :: messages :: id :: Select) } ; (@ selection_field_to_selection_param ; message) => { Into :: < crate :: prisma :: messages :: SelectParam > :: into (crate :: prisma :: messages :: message :: Select) } ; (@ selection_field_to_selection_param ; created_at) => { Into :: < crate :: prisma :: messages :: SelectParam > :: into (crate :: prisma :: messages :: created_at :: Select) } ; (@ selection_field_to_selection_param ; updated_at) => { Into :: < crate :: prisma :: messages :: SelectParam > :: into (crate :: prisma :: messages :: updated_at :: Select) } ; (@ selection_field_to_selection_param ; user $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? : $ selection_mode : ident { $ ($ selections : tt) + }) => { { Into :: < crate :: prisma :: messages :: SelectParam > :: into (crate :: prisma :: messages :: user :: Select :: $ selection_mode (crate :: prisma :: user :: select ! (@ selections_to_params ; : $ selection_mode { $ ($ selections) + }) . into_iter () . collect ())) } } ; (@ selection_field_to_selection_param ; user $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ?) => { { Into :: < crate :: prisma :: messages :: SelectParam > :: into (crate :: prisma :: messages :: user :: Select :: Fetch) } } ; (@ selection_field_to_selection_param ; user_id) => { Into :: < crate :: prisma :: messages :: SelectParam > :: into (crate :: prisma :: messages :: user_id :: Select) } ; (@ selection_field_to_selection_param ; room $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? : $ selection_mode : ident { $ ($ selections : tt) + }) => { { Into :: < crate :: prisma :: messages :: SelectParam > :: into (crate :: prisma :: messages :: room :: Select :: $ selection_mode (crate :: prisma :: rooms :: select ! (@ selections_to_params ; : $ selection_mode { $ ($ selections) + }) . into_iter () . collect ())) } } ; (@ selection_field_to_selection_param ; room $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ?) => { { Into :: < crate :: prisma :: messages :: SelectParam > :: into (crate :: prisma :: messages :: room :: Select :: Fetch) } } ; (@ selection_field_to_selection_param ; room_id) => { Into :: < crate :: prisma :: messages :: SelectParam > :: into (crate :: prisma :: messages :: room_id :: Select) } ; (@ selection_field_to_selection_param ; $ ($ tokens : tt) *) => { compile_error ! (stringify ! ($ ($ tokens) *)) } ; (@ selections_to_params ; : $ macro_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { [$ (crate :: prisma :: messages :: $ macro_name ! (@ selection_field_to_selection_param ; $ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) ,) +] } ; (@ filters_to_args ;) => { vec ! [] } ; (@ filters_to_args ; $ ($ t : tt) *) => { $ ($ t) * } ; (@ field_serde_name ; id) => { "id" } ; (@ field_serde_name ; message) => { "message" } ; (@ field_serde_name ; created_at) => { "createdAt" } ; (@ field_serde_name ; updated_at) => { "updatedAt" } ; (@ field_serde_name ; user) => { "user" } ; (@ field_serde_name ; user_id) => { "userId" } ; (@ field_serde_name ; room) => { "room" } ; (@ field_serde_name ; room_id) => { "roomId" } ; }
    pub use _select_messages as select;
    pub enum SelectParam {
        Id(id::Select),
        Message(message::Select),
        CreatedAt(created_at::Select),
        UpdatedAt(updated_at::Select),
        User(user::Select),
        UserId(user_id::Select),
        Room(room::Select),
        RoomId(room_id::Select),
    }
    impl SelectParam {
        pub fn to_selection(self) -> ::prisma_client_rust::Selection {
            match self {
                Self::Id(data) => data.to_selection(),
                Self::Message(data) => data.to_selection(),
                Self::CreatedAt(data) => data.to_selection(),
                Self::UpdatedAt(data) => data.to_selection(),
                Self::User(data) => data.to_selection(),
                Self::UserId(data) => data.to_selection(),
                Self::Room(data) => data.to_selection(),
                Self::RoomId(data) => data.to_selection(),
            }
        }
    }
    #[macro_export]
    macro_rules ! _include_messages { ($ (($ ($ func_arg : ident : $ func_arg_ty : ty) , +) =>) ? $ module_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { # [allow (warnings)] pub mod $ module_name { crate :: prisma :: messages :: include ! (@ definitions ; $ module_name ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; use super :: * ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: IncludeType for Selection { type Data = Data ; type ModelData = crate :: prisma :: messages :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } pub fn include ($ ($ ($ func_arg : $ func_arg_ty) , +) ?) -> Selection { Selection ([crate :: prisma :: messages :: include ! (@ selections_to_params ; : include { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () , < crate :: prisma :: messages :: Types as :: prisma_client_rust :: ModelTypes > :: scalar_selections ()] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } } ; ({ $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { { crate :: prisma :: messages :: include ! (@ definitions ; ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: IncludeType for Selection { type Data = Data ; type ModelData = crate :: prisma :: messages :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } Selection ([crate :: prisma :: messages :: include ! (@ selections_to_params ; : include { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () , < crate :: prisma :: messages :: Types as :: prisma_client_rust :: ModelTypes > :: scalar_selections ()] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } ; (@ definitions ; $ ($ module_name : ident) ? ; $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) +) => { # [allow (warnings)] enum Fields { user , room } # [allow (warnings)] impl Fields { fn selections () { $ (let _ = Fields :: $ field ;) + } } # [allow (warnings)] # [derive (std :: fmt :: Debug , Clone)] pub struct Data { pub id : i32 , pub message : String , pub created_at : :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > , pub updated_at : :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > , pub user_id : i32 , pub room_id : i32 , $ (pub $ field : crate :: prisma :: messages :: include ! (@ field_type ; $ field $ (: $ selection_mode { $ ($ selections) + }) ?) ,) + } impl :: serde :: Serialize for Data { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : :: serde :: Serializer , { use :: serde :: ser :: SerializeStruct ; let mut state = serializer . serialize_struct ("Data" , [$ (stringify ! ($ field) ,) + stringify ! (id) , stringify ! (message) , stringify ! (created_at) , stringify ! (updated_at) , stringify ! (user_id) , stringify ! (room_id)] . len ()) ? ; $ (state . serialize_field (crate :: prisma :: messages :: $ field :: NAME , & self . $ field) ? ;) * state . serialize_field (crate :: prisma :: messages :: id :: NAME , & self . id) ? ; state . serialize_field (crate :: prisma :: messages :: message :: NAME , & self . message) ? ; state . serialize_field (crate :: prisma :: messages :: created_at :: NAME , & self . created_at) ? ; state . serialize_field (crate :: prisma :: messages :: updated_at :: NAME , & self . updated_at) ? ; state . serialize_field (crate :: prisma :: messages :: user_id :: NAME , & self . user_id) ? ; state . serialize_field (crate :: prisma :: messages :: room_id :: NAME , & self . room_id) ? ; state . end () } } impl < 'de > :: serde :: Deserialize < 'de > for Data { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : :: serde :: Deserializer < 'de > , { # [allow (warnings)] enum Field { $ ($ field) , + , id , message , created_at , updated_at , user_id , room_id } impl < 'de > :: serde :: Deserialize < 'de > for Field { fn deserialize < D > (deserializer : D) -> Result < Field , D :: Error > where D : :: serde :: Deserializer < 'de > , { struct FieldVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for FieldVisitor { type Value = Field ; fn expecting (& self , formatter : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { formatter . write_str (& [$ (crate :: prisma :: messages :: $ field :: NAME) , + , crate :: prisma :: messages :: id :: NAME , crate :: prisma :: messages :: message :: NAME , crate :: prisma :: messages :: created_at :: NAME , crate :: prisma :: messages :: updated_at :: NAME , crate :: prisma :: messages :: user_id :: NAME , crate :: prisma :: messages :: room_id :: NAME] . into_iter () . collect :: < Vec < _ >> () . join (", ")) } fn visit_str < E > (self , value : & str) -> Result < Field , E > where E : :: serde :: de :: Error , { match value { $ (crate :: prisma :: messages :: $ field :: NAME => Ok (Field :: $ field)) , * , crate :: prisma :: messages :: id :: NAME => Ok (Field :: id) , crate :: prisma :: messages :: message :: NAME => Ok (Field :: message) , crate :: prisma :: messages :: created_at :: NAME => Ok (Field :: created_at) , crate :: prisma :: messages :: updated_at :: NAME => Ok (Field :: updated_at) , crate :: prisma :: messages :: user_id :: NAME => Ok (Field :: user_id) , crate :: prisma :: messages :: room_id :: NAME => Ok (Field :: room_id) , _ => Err (:: serde :: de :: Error :: unknown_field (value , FIELDS)) , } } } deserializer . deserialize_identifier (FieldVisitor) } } struct DataVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for DataVisitor { type Value = Data ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("struct Data") } fn visit_map < V > (self , mut map : V) -> Result < Data , V :: Error > where V : :: serde :: de :: MapAccess < 'de > , { $ (let mut $ field = None ;) * let mut id = None ; let mut message = None ; let mut created_at = None ; let mut updated_at = None ; let mut user_id = None ; let mut room_id = None ; while let Some (key) = map . next_key () ? { match key { Field :: id => { if id . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: messages :: id :: NAME)) ; } id = Some (map . next_value () ?) ; } Field :: message => { if message . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: messages :: message :: NAME)) ; } message = Some (map . next_value () ?) ; } Field :: created_at => { if created_at . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: messages :: created_at :: NAME)) ; } created_at = Some (map . next_value () ?) ; } Field :: updated_at => { if updated_at . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: messages :: updated_at :: NAME)) ; } updated_at = Some (map . next_value () ?) ; } Field :: user_id => { if user_id . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: messages :: user_id :: NAME)) ; } user_id = Some (map . next_value () ?) ; } Field :: room_id => { if room_id . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: messages :: room_id :: NAME)) ; } room_id = Some (map . next_value () ?) ; } $ (Field :: $ field => { if $ field . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: messages :: $ field :: NAME)) ; } $ field = Some (map . next_value () ?) ; }) * } } $ (let $ field = $ field . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: messages :: $ field :: NAME)) ? ;) * let id = id . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: messages :: id :: NAME)) ? ; let message = message . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: messages :: message :: NAME)) ? ; let created_at = created_at . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: messages :: created_at :: NAME)) ? ; let updated_at = updated_at . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: messages :: updated_at :: NAME)) ? ; let user_id = user_id . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: messages :: user_id :: NAME)) ? ; let room_id = room_id . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: messages :: room_id :: NAME)) ? ; Ok (Data { id , message , created_at , updated_at , user_id , room_id , $ ($ field) , * }) } } const FIELDS : & 'static [& 'static str] = & ["id" , "message" , "createdAt" , "updatedAt" , "user" , "userId" , "room" , "roomId"] ; deserializer . deserialize_struct ("Data" , FIELDS , DataVisitor) } } $ ($ (pub mod $ field { crate :: prisma :: messages :: $ selection_mode ! (@ field_module ; $ field : $ selection_mode { $ ($ selections) + }) ; }) ?) + } ; (@ field_type ; user : $ selection_mode : ident { $ ($ selections : tt) + }) => { user :: Data } ; (@ field_type ; user) => { crate :: prisma :: user :: Data } ; (@ field_type ; room : $ selection_mode : ident { $ ($ selections : tt) + }) => { room :: Data } ; (@ field_type ; room) => { crate :: prisma :: rooms :: Data } ; (@ field_type ; $ field : ident $ ($ tokens : tt) *) => { compile_error ! (stringify ! (Cannot include nonexistent relation $ field on model "Messages" , available relations are "user, room")) } ; (@ field_module ; user : $ selection_mode : ident { $ ($ selections : tt) + }) => { crate :: prisma :: user :: include ! (@ definitions ; ; $ ($ selections) +) ; } ; (@ field_module ; room : $ selection_mode : ident { $ ($ selections : tt) + }) => { crate :: prisma :: rooms :: include ! (@ definitions ; ; $ ($ selections) +) ; } ; (@ field_module ; $ ($ tokens : tt) *) => { } ; (@ selection_field_to_selection_param ; user $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? : $ selection_mode : ident { $ ($ selections : tt) + }) => { { Into :: < crate :: prisma :: messages :: IncludeParam > :: into (crate :: prisma :: messages :: user :: Include :: $ selection_mode (crate :: prisma :: user :: select ! (@ selections_to_params ; : $ selection_mode { $ ($ selections) + }) . into_iter () . collect ())) } } ; (@ selection_field_to_selection_param ; user $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ?) => { { Into :: < crate :: prisma :: messages :: IncludeParam > :: into (crate :: prisma :: messages :: user :: Include :: Fetch) } } ; (@ selection_field_to_selection_param ; room $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? : $ selection_mode : ident { $ ($ selections : tt) + }) => { { Into :: < crate :: prisma :: messages :: IncludeParam > :: into (crate :: prisma :: messages :: room :: Include :: $ selection_mode (crate :: prisma :: rooms :: select ! (@ selections_to_params ; : $ selection_mode { $ ($ selections) + }) . into_iter () . collect ())) } } ; (@ selection_field_to_selection_param ; room $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ?) => { { Into :: < crate :: prisma :: messages :: IncludeParam > :: into (crate :: prisma :: messages :: room :: Include :: Fetch) } } ; (@ selection_field_to_selection_param ; $ ($ tokens : tt) *) => { compile_error ! (stringify ! ($ ($ tokens) *)) } ; (@ selections_to_params ; : $ macro_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { [$ (crate :: prisma :: messages :: $ macro_name ! (@ selection_field_to_selection_param ; $ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) ,) +] } ; (@ filters_to_args ;) => { vec ! [] } ; (@ filters_to_args ; $ ($ t : tt) *) => { $ ($ t) * } ; (@ field_serde_name ; id) => { "id" } ; (@ field_serde_name ; message) => { "message" } ; (@ field_serde_name ; created_at) => { "createdAt" } ; (@ field_serde_name ; updated_at) => { "updatedAt" } ; (@ field_serde_name ; user) => { "user" } ; (@ field_serde_name ; user_id) => { "userId" } ; (@ field_serde_name ; room) => { "room" } ; (@ field_serde_name ; room_id) => { "roomId" } ; }
    pub use _include_messages as include;
    pub enum IncludeParam {
        Id(id::Include),
        Message(message::Include),
        CreatedAt(created_at::Include),
        UpdatedAt(updated_at::Include),
        User(user::Include),
        UserId(user_id::Include),
        Room(room::Include),
        RoomId(room_id::Include),
    }
    impl IncludeParam {
        pub fn to_selection(self) -> ::prisma_client_rust::Selection {
            match self {
                Self::Id(data) => data.to_selection(),
                Self::Message(data) => data.to_selection(),
                Self::CreatedAt(data) => data.to_selection(),
                Self::UpdatedAt(data) => data.to_selection(),
                Self::User(data) => data.to_selection(),
                Self::UserId(data) => data.to_selection(),
                Self::Room(data) => data.to_selection(),
                Self::RoomId(data) => data.to_selection(),
            }
        }
    }
    #[macro_export]
    macro_rules ! _partial_unchecked_messages { ($ struct_name : ident { $ ($ scalar_field : ident) + }) => { :: prisma_client_rust :: macros :: partial_unchecked ! { crate :: prisma :: messages struct $ struct_name { # [serde (rename = "id")] pub id : i32 , # [serde (rename = "message")] pub message : String , # [serde (rename = "createdAt")] pub created_at : :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > , # [serde (rename = "updatedAt")] pub updated_at : :: prisma_client_rust :: chrono :: DateTime < :: prisma_client_rust :: chrono :: FixedOffset , > , # [serde (rename = "userId")] pub user_id : i32 , # [serde (rename = "roomId")] pub room_id : i32 } [$ ($ scalar_field) , +] } } ; }
    pub use _partial_unchecked_messages as partial_unchecked;
    #[derive(Debug, Clone, :: serde :: Serialize, :: serde :: Deserialize)]
    pub struct Data {
        #[serde(rename = "id")]
        pub id: i32,
        #[serde(rename = "message")]
        pub message: String,
        #[serde(rename = "createdAt")]
        pub created_at:
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        #[serde(rename = "updatedAt")]
        pub updated_at:
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        #[serde(rename = "user")]
        pub user: Option<Box<super::user::Data>>,
        #[serde(rename = "userId")]
        pub user_id: i32,
        #[serde(rename = "room")]
        pub room: Option<Box<super::rooms::Data>>,
        #[serde(rename = "roomId")]
        pub room_id: i32,
    }
    impl Data {
        pub fn user(
            &self,
        ) -> Result<&super::user::Data, ::prisma_client_rust::RelationNotFetchedError> {
            self.user
                .as_ref()
                .ok_or(::prisma_client_rust::RelationNotFetchedError::new(
                    stringify!(user),
                ))
                .map(|v| v.as_ref())
        }
        pub fn room(
            &self,
        ) -> Result<&super::rooms::Data, ::prisma_client_rust::RelationNotFetchedError> {
            self.room
                .as_ref()
                .ok_or(::prisma_client_rust::RelationNotFetchedError::new(
                    stringify!(room),
                ))
                .map(|v| v.as_ref())
        }
    }
    #[derive(Clone)]
    pub enum WithParam {
        User(super::user::UniqueArgs),
        Room(super::rooms::UniqueArgs),
    }
    impl Into<::prisma_client_rust::Selection> for WithParam {
        fn into(self) -> ::prisma_client_rust::Selection {
            match self {
                Self::User(args) => {
                    let mut selections =
                        <super::user::Types as ::prisma_client_rust::ModelTypes>::scalar_selections(
                        );
                    selections.extend(
                        args.with_params
                            .into_iter()
                            .map(Into::<::prisma_client_rust::Selection>::into),
                    );
                    ::prisma_client_rust::Selection::new(user::NAME, None, [], selections)
                }
                Self::Room(args) => {
                    let mut selections = < super :: rooms :: Types as :: prisma_client_rust :: ModelTypes > :: scalar_selections () ;
                    selections.extend(
                        args.with_params
                            .into_iter()
                            .map(Into::<::prisma_client_rust::Selection>::into),
                    );
                    ::prisma_client_rust::Selection::new(room::NAME, None, [], selections)
                }
            }
        }
    }
    #[derive(Clone)]
    pub enum SetParam {
        SetId(i32),
        IncrementId(i32),
        DecrementId(i32),
        MultiplyId(i32),
        DivideId(i32),
        SetMessage(String),
        SetCreatedAt(
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        ),
        SetUpdatedAt(
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        ),
        ConnectUser(super::user::UniqueWhereParam),
        SetUserId(i32),
        IncrementUserId(i32),
        DecrementUserId(i32),
        MultiplyUserId(i32),
        DivideUserId(i32),
        ConnectRoom(super::rooms::UniqueWhereParam),
        SetRoomId(i32),
        IncrementRoomId(i32),
        DecrementRoomId(i32),
        MultiplyRoomId(i32),
        DivideRoomId(i32),
    }
    impl From<SetParam> for (String, ::prisma_client_rust::PrismaValue) {
        fn from(param: SetParam) -> Self {
            match param {
                SetParam::SetId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Int(value as i64),
                ),
                SetParam::IncrementId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "increment".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::DecrementId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "decrement".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::MultiplyId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "multiply".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::DivideId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "divide".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::SetMessage(value) => (
                    message::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(value),
                ),
                SetParam::SetCreatedAt(value) => (
                    created_at::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::DateTime(value),
                ),
                SetParam::SetUpdatedAt(value) => (
                    updated_at::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::DateTime(value),
                ),
                SetParam::ConnectUser(where_param) => (
                    user::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "connect".to_string(),
                        ::prisma_client_rust::PrismaValue::Object(
                            [where_param]
                                .into_iter()
                                .map(Into::<super::user::WhereParam>::into)
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .collect(),
                        ),
                    )]),
                ),
                SetParam::SetUserId(value) => (
                    user_id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Int(value as i64),
                ),
                SetParam::IncrementUserId(value) => (
                    user_id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "increment".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::DecrementUserId(value) => (
                    user_id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "decrement".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::MultiplyUserId(value) => (
                    user_id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "multiply".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::DivideUserId(value) => (
                    user_id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "divide".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::ConnectRoom(where_param) => (
                    room::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "connect".to_string(),
                        ::prisma_client_rust::PrismaValue::Object(
                            [where_param]
                                .into_iter()
                                .map(Into::<super::rooms::WhereParam>::into)
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .collect(),
                        ),
                    )]),
                ),
                SetParam::SetRoomId(value) => (
                    room_id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Int(value as i64),
                ),
                SetParam::IncrementRoomId(value) => (
                    room_id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "increment".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::DecrementRoomId(value) => (
                    room_id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "decrement".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::MultiplyRoomId(value) => (
                    room_id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "multiply".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::DivideRoomId(value) => (
                    room_id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::Object(vec![(
                        "divide".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                ),
            }
        }
    }
    #[derive(Clone)]
    pub enum UncheckedSetParam {
        Id(i32),
        Message(String),
        CreatedAt(
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        ),
        UpdatedAt(
            ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
        ),
        UserId(i32),
        RoomId(i32),
    }
    impl From<UncheckedSetParam> for SetParam {
        fn from(param: UncheckedSetParam) -> Self {
            match param {
                UncheckedSetParam::Id(value) => Self::SetId(value),
                UncheckedSetParam::Message(value) => Self::SetMessage(value),
                UncheckedSetParam::CreatedAt(value) => Self::SetCreatedAt(value),
                UncheckedSetParam::UpdatedAt(value) => Self::SetUpdatedAt(value),
                UncheckedSetParam::UserId(value) => Self::SetUserId(value),
                UncheckedSetParam::RoomId(value) => Self::SetRoomId(value),
            }
        }
    }
    #[derive(Clone)]
    pub enum OrderByParam {
        Id(::prisma_client_rust::Direction),
        Message(::prisma_client_rust::Direction),
        CreatedAt(::prisma_client_rust::Direction),
        UpdatedAt(::prisma_client_rust::Direction),
        UserId(::prisma_client_rust::Direction),
        RoomId(::prisma_client_rust::Direction),
    }
    impl Into<(String, ::prisma_client_rust::PrismaValue)> for OrderByParam {
        fn into(self) -> (String, ::prisma_client_rust::PrismaValue) {
            match self {
                Self::Id(direction) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
                Self::Message(direction) => (
                    message::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
                Self::CreatedAt(direction) => (
                    created_at::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
                Self::UpdatedAt(direction) => (
                    updated_at::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
                Self::UserId(direction) => (
                    user_id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
                Self::RoomId(direction) => (
                    room_id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
            }
        }
    }
    #[derive(Clone)]
    pub enum WhereParam {
        Not(Vec<WhereParam>),
        Or(Vec<WhereParam>),
        And(Vec<WhereParam>),
        Id(_prisma::read_filters::IntFilter),
        Message(_prisma::read_filters::StringFilter),
        CreatedAt(_prisma::read_filters::DateTimeFilter),
        UpdatedAt(_prisma::read_filters::DateTimeFilter),
        UserIs(Vec<super::user::WhereParam>),
        UserIsNot(Vec<super::user::WhereParam>),
        UserId(_prisma::read_filters::IntFilter),
        RoomIs(Vec<super::rooms::WhereParam>),
        RoomIsNot(Vec<super::rooms::WhereParam>),
        RoomId(_prisma::read_filters::IntFilter),
    }
    impl ::prisma_client_rust::WhereInput for WhereParam {
        fn serialize(self) -> ::prisma_client_rust::SerializedWhereInput {
            let (name, value) = match self {
                Self::Not(value) => (
                    "NOT",
                    ::prisma_client_rust::SerializedWhereValue::Object(
                        ::prisma_client_rust::merge_fields(
                            value
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(Into::into)
                                .collect(),
                        ),
                    ),
                ),
                Self::Or(value) => (
                    "OR",
                    ::prisma_client_rust::SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(::prisma_client_rust::WhereInput::serialize)
                            .map(Into::into)
                            .map(|v| vec![v])
                            .map(::prisma_client_rust::PrismaValue::Object)
                            .collect(),
                    ),
                ),
                Self::And(value) => (
                    "AND",
                    ::prisma_client_rust::SerializedWhereValue::Object(
                        ::prisma_client_rust::merge_fields(
                            value
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(Into::into)
                                .collect(),
                        ),
                    ),
                ),
                Self::Id(value) => (id::NAME, value.into()),
                Self::Message(value) => (message::NAME, value.into()),
                Self::CreatedAt(value) => (created_at::NAME, value.into()),
                Self::UpdatedAt(value) => (updated_at::NAME, value.into()),
                Self::UserIs(where_params) => (
                    user::NAME,
                    ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "is".to_string(),
                        ::prisma_client_rust::PrismaValue::Object(
                            where_params
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .collect(),
                        ),
                    )]),
                ),
                Self::UserIsNot(where_params) => (
                    user::NAME,
                    ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "isNot".to_string(),
                        ::prisma_client_rust::PrismaValue::Object(
                            where_params
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .collect(),
                        ),
                    )]),
                ),
                Self::UserId(value) => (user_id::NAME, value.into()),
                Self::RoomIs(where_params) => (
                    room::NAME,
                    ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "is".to_string(),
                        ::prisma_client_rust::PrismaValue::Object(
                            where_params
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .collect(),
                        ),
                    )]),
                ),
                Self::RoomIsNot(where_params) => (
                    room::NAME,
                    ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "isNot".to_string(),
                        ::prisma_client_rust::PrismaValue::Object(
                            where_params
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(::prisma_client_rust::SerializedWhereInput::transform_equals)
                                .collect(),
                        ),
                    )]),
                ),
                Self::RoomId(value) => (room_id::NAME, value.into()),
            };
            ::prisma_client_rust::SerializedWhereInput::new(name, value.into())
        }
    }
    #[derive(Clone)]
    pub enum UniqueWhereParam {
        IdEquals(i32),
    }
    impl From<UniqueWhereParam> for WhereParam {
        fn from(value: UniqueWhereParam) -> Self {
            match value {
                UniqueWhereParam::IdEquals(value) => {
                    Self::Id(_prisma::read_filters::IntFilter::Equals(value))
                }
            }
        }
    }
    impl From<::prisma_client_rust::Operator<Self>> for WhereParam {
        fn from(op: ::prisma_client_rust::Operator<Self>) -> Self {
            match op {
                ::prisma_client_rust::Operator::Not(value) => Self::Not(value),
                ::prisma_client_rust::Operator::And(value) => Self::And(value),
                ::prisma_client_rust::Operator::Or(value) => Self::Or(value),
            }
        }
    }
    #[derive(Clone)]
    pub struct Types;
    impl ::prisma_client_rust::ModelTypes for Types {
        type Data = Data;
        type Where = WhereParam;
        type UncheckedSet = UncheckedSetParam;
        type Set = SetParam;
        type With = WithParam;
        type OrderBy = OrderByParam;
        type Cursor = UniqueWhereParam;
        const MODEL: &'static str = NAME;
        fn scalar_selections() -> Vec<::prisma_client_rust::Selection> {
            vec![
                ::prisma_client_rust::sel(id::NAME),
                ::prisma_client_rust::sel(message::NAME),
                ::prisma_client_rust::sel(created_at::NAME),
                ::prisma_client_rust::sel(updated_at::NAME),
                ::prisma_client_rust::sel(user_id::NAME),
                ::prisma_client_rust::sel(room_id::NAME),
            ]
        }
    }
    pub type UniqueArgs = ::prisma_client_rust::UniqueArgs<Types>;
    pub type ManyArgs = ::prisma_client_rust::ManyArgs<Types>;
    pub type Count<'a> = ::prisma_client_rust::Count<'a, Types>;
    pub type Create<'a> = ::prisma_client_rust::Create<'a, Types>;
    pub type CreateMany<'a> = ::prisma_client_rust::CreateMany<'a, Types>;
    pub type FindUnique<'a> = ::prisma_client_rust::FindUnique<'a, Types>;
    pub type FindMany<'a> = ::prisma_client_rust::FindMany<'a, Types>;
    pub type FindFirst<'a> = ::prisma_client_rust::FindFirst<'a, Types>;
    pub type Update<'a> = ::prisma_client_rust::Update<'a, Types>;
    pub type UpdateMany<'a> = ::prisma_client_rust::UpdateMany<'a, Types>;
    pub type Upsert<'a> = ::prisma_client_rust::Upsert<'a, Types>;
    pub type Delete<'a> = ::prisma_client_rust::Delete<'a, Types>;
    pub type DeleteMany<'a> = ::prisma_client_rust::DeleteMany<'a, Types>;
    #[derive(Clone)]
    pub struct Actions<'a> {
        pub client: &'a ::prisma_client_rust::PrismaClientInternals,
    }
    impl<'a> Actions<'a> {
        pub fn find_unique(self, _where: UniqueWhereParam) -> FindUnique<'a> {
            FindUnique::new(self.client, _where.into())
        }
        pub fn find_first(self, _where: Vec<WhereParam>) -> FindFirst<'a> {
            FindFirst::new(self.client, _where)
        }
        pub fn find_many(self, _where: Vec<WhereParam>) -> FindMany<'a> {
            FindMany::new(self.client, _where)
        }
        pub fn create(
            self,
            message: String,
            user: super::user::UniqueWhereParam,
            room: super::rooms::UniqueWhereParam,
            mut _params: Vec<SetParam>,
        ) -> Create<'a> {
            _params.extend([
                message::set(message),
                user::connect(user),
                room::connect(room),
            ]);
            Create::new(self.client, _params)
        }
        pub fn create_unchecked(
            self,
            message: String,
            user_id: i32,
            room_id: i32,
            mut _params: Vec<UncheckedSetParam>,
        ) -> Create<'a> {
            _params.extend([
                message::set(message),
                user_id::set(user_id),
                room_id::set(room_id),
            ]);
            Create::new(self.client, _params.into_iter().map(Into::into).collect())
        }
        pub fn create_many(self, data: Vec<(String, i32, i32, Vec<SetParam>)>) -> CreateMany<'a> {
            let data = data
                .into_iter()
                .map(|(message, user_id, room_id, mut _params)| {
                    _params.extend([
                        message::set(message),
                        user_id::set(user_id),
                        room_id::set(room_id),
                    ]);
                    _params
                })
                .collect();
            CreateMany::new(self.client, data)
        }
        pub fn update(self, _where: UniqueWhereParam, _params: Vec<SetParam>) -> Update<'a> {
            Update::new(self.client, _where.into(), _params, vec![])
        }
        pub fn update_unchecked(
            self,
            _where: UniqueWhereParam,
            _params: Vec<UncheckedSetParam>,
        ) -> Update<'a> {
            Update::new(
                self.client,
                _where.into(),
                _params.into_iter().map(Into::into).collect(),
                vec![],
            )
        }
        pub fn update_many(
            self,
            _where: Vec<WhereParam>,
            _params: Vec<SetParam>,
        ) -> UpdateMany<'a> {
            UpdateMany::new(self.client, _where, _params)
        }
        pub fn upsert(
            self,
            _where: UniqueWhereParam,
            (message, user, room, mut _params): (
                String,
                super::user::UniqueWhereParam,
                super::rooms::UniqueWhereParam,
                Vec<SetParam>,
            ),
            _update: Vec<SetParam>,
        ) -> Upsert<'a> {
            _params.extend([
                message::set(message),
                user::connect(user),
                room::connect(room),
            ]);
            Upsert::new(self.client, _where.into(), _params, _update)
        }
        pub fn delete(self, _where: UniqueWhereParam) -> Delete<'a> {
            Delete::new(self.client, _where.into(), vec![])
        }
        pub fn delete_many(self, _where: Vec<WhereParam>) -> DeleteMany<'a> {
            DeleteMany::new(self.client, _where)
        }
        pub fn count(self, _where: Vec<WhereParam>) -> Count<'a> {
            Count::new(self.client, _where)
        }
        pub fn find_raw<T: ::prisma_client_rust::Data>(
            self,
        ) -> ::prisma_client_rust::FindRaw<'a, Types, T> {
            ::prisma_client_rust::FindRaw::new(self.client)
        }
        pub fn aggregate_raw<T: ::prisma_client_rust::Data>(
            self,
        ) -> ::prisma_client_rust::AggregateRaw<'a, Types, T> {
            ::prisma_client_rust::AggregateRaw::new(self.client)
        }
    }
}
pub mod _prisma {
    pub struct PrismaClientBuilder {
        url: Option<String>,
        action_notifier: ::prisma_client_rust::ActionNotifier,
    }
    impl PrismaClientBuilder {
        fn new() -> Self {
            Self {
                url: None,
                action_notifier: ::prisma_client_rust::ActionNotifier::new(),
            }
        }
        pub fn with_url(mut self, url: String) -> Self {
            self.url = Some(url);
            self
        }
        pub async fn build(self) -> Result<PrismaClient, ::prisma_client_rust::NewClientError> {
            let internals = ::prisma_client_rust::PrismaClientInternals::new(
                self.url,
                self.action_notifier,
                super::DATAMODEL_STR,
            )
            .await?;
            Ok(PrismaClient(internals))
        }
    }
    pub struct PrismaClient(::prisma_client_rust::PrismaClientInternals);
    impl ::std::fmt::Debug for PrismaClient {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("PrismaClient").finish()
        }
    }
    impl PrismaClient {
        pub fn _builder() -> PrismaClientBuilder {
            PrismaClientBuilder::new()
        }
        pub fn _query_raw<T: ::prisma_client_rust::Data>(
            &self,
            query: ::prisma_client_rust::Raw,
        ) -> ::prisma_client_rust::QueryRaw<T> {
            ::prisma_client_rust::QueryRaw::new(&self.0, query, super::DATABASE_STR)
        }
        pub fn _execute_raw(
            &self,
            query: ::prisma_client_rust::Raw,
        ) -> ::prisma_client_rust::ExecuteRaw {
            ::prisma_client_rust::ExecuteRaw::new(&self.0, query, super::DATABASE_STR)
        }
        pub async fn _batch<
            'batch,
            T: ::prisma_client_rust::BatchContainer<'batch, Marker>,
            Marker,
        >(
            &self,
            queries: T,
        ) -> ::prisma_client_rust::Result<
            <T as ::prisma_client_rust::BatchContainer<'batch, Marker>>::ReturnType,
        > {
            ::prisma_client_rust::batch(queries, &self.0).await
        }
        pub fn _transaction(&self) -> ::prisma_client_rust::TransactionBuilder<Self> {
            ::prisma_client_rust::TransactionBuilder::_new(self, &self.0)
        }
        pub fn user(&self) -> super::user::Actions {
            super::user::Actions { client: &self.0 }
        }
        pub fn rooms(&self) -> super::rooms::Actions {
            super::rooms::Actions { client: &self.0 }
        }
        pub fn messages(&self) -> super::messages::Actions {
            super::messages::Actions { client: &self.0 }
        }
    }
    impl ::prisma_client_rust::PrismaClient for PrismaClient {
        fn internals(&self) -> &::prisma_client_rust::PrismaClientInternals {
            &self.0
        }
        fn internals_mut(&mut self) -> &mut ::prisma_client_rust::PrismaClientInternals {
            &mut self.0
        }
        fn with_tx_id(&self, tx_id: Option<::prisma_client_rust::query_core::TxId>) -> Self {
            Self(self.0.with_tx_id(tx_id))
        }
    }
    #[derive(Debug, Clone, Copy, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Eq)]
    pub enum MessagesScalarFieldEnum {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "message")]
        Message,
        #[serde(rename = "createdAt")]
        CreatedAt,
        #[serde(rename = "updatedAt")]
        UpdatedAt,
        #[serde(rename = "userId")]
        UserId,
        #[serde(rename = "roomId")]
        RoomId,
    }
    impl ToString for MessagesScalarFieldEnum {
        fn to_string(&self) -> String {
            match self {
                Self::Id => "id".to_string(),
                Self::Message => "message".to_string(),
                Self::CreatedAt => "createdAt".to_string(),
                Self::UpdatedAt => "updatedAt".to_string(),
                Self::UserId => "userId".to_string(),
                Self::RoomId => "roomId".to_string(),
            }
        }
    }
    #[derive(Debug, Clone, Copy, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Eq)]
    pub enum RoomsScalarFieldEnum {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "createdAt")]
        CreatedAt,
        #[serde(rename = "updatedAt")]
        UpdatedAt,
    }
    impl ToString for RoomsScalarFieldEnum {
        fn to_string(&self) -> String {
            match self {
                Self::Id => "id".to_string(),
                Self::Name => "name".to_string(),
                Self::CreatedAt => "createdAt".to_string(),
                Self::UpdatedAt => "updatedAt".to_string(),
            }
        }
    }
    #[derive(Debug, Clone, Copy, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Eq)]
    pub enum SortOrder {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
    }
    impl ToString for SortOrder {
        fn to_string(&self) -> String {
            match self {
                Self::Asc => "asc".to_string(),
                Self::Desc => "desc".to_string(),
            }
        }
    }
    #[derive(Debug, Clone, Copy, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Eq)]
    pub enum TransactionIsolationLevel {
        #[serde(rename = "ReadUncommitted")]
        ReadUncommitted,
        #[serde(rename = "ReadCommitted")]
        ReadCommitted,
        #[serde(rename = "RepeatableRead")]
        RepeatableRead,
        #[serde(rename = "Serializable")]
        Serializable,
    }
    impl ToString for TransactionIsolationLevel {
        fn to_string(&self) -> String {
            match self {
                Self::ReadUncommitted => "ReadUncommitted".to_string(),
                Self::ReadCommitted => "ReadCommitted".to_string(),
                Self::RepeatableRead => "RepeatableRead".to_string(),
                Self::Serializable => "Serializable".to_string(),
            }
        }
    }
    impl ::prisma_client_rust::TransactionIsolationLevel for TransactionIsolationLevel {}
    #[derive(Debug, Clone, Copy, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Eq)]
    pub enum UserScalarFieldEnum {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "email")]
        Email,
        #[serde(rename = "name")]
        Name,
        #[serde(rename = "ip")]
        Ip,
        #[serde(rename = "createdAt")]
        CreatedAt,
        #[serde(rename = "updatedAt")]
        UpdatedAt,
    }
    impl ToString for UserScalarFieldEnum {
        fn to_string(&self) -> String {
            match self {
                Self::Id => "id".to_string(),
                Self::Email => "email".to_string(),
                Self::Name => "name".to_string(),
                Self::Ip => "ip".to_string(),
                Self::CreatedAt => "createdAt".to_string(),
                Self::UpdatedAt => "updatedAt".to_string(),
            }
        }
    }
    pub mod read_filters {
        #[derive(Clone)]
        pub enum IntFilter {
            Equals(i32),
            InVec(Vec<i32>),
            NotInVec(Vec<i32>),
            Lt(i32),
            Lte(i32),
            Gt(i32),
            Gte(i32),
            Not(i32),
        }
        impl Into<::prisma_client_rust::SerializedWhereValue> for IntFilter {
            fn into(self) -> ::prisma_client_rust::SerializedWhereValue {
                match self {
                    Self::Equals(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "equals".to_string(),
                            ::prisma_client_rust::PrismaValue::Int(value as i64),
                        )])
                    }
                    Self::InVec(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "in".to_string(),
                            ::prisma_client_rust::PrismaValue::List(
                                value
                                    .into_iter()
                                    .map(|value| {
                                        ::prisma_client_rust::PrismaValue::Int(value as i64)
                                    })
                                    .collect(),
                            ),
                        )])
                    }
                    Self::NotInVec(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "notIn".to_string(),
                            ::prisma_client_rust::PrismaValue::List(
                                value
                                    .into_iter()
                                    .map(|value| {
                                        ::prisma_client_rust::PrismaValue::Int(value as i64)
                                    })
                                    .collect(),
                            ),
                        )])
                    }
                    Self::Lt(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "lt".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                    Self::Lte(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "lte".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                    Self::Gt(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "gt".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                    Self::Gte(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "gte".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                    Self::Not(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "not".to_string(),
                        ::prisma_client_rust::PrismaValue::Int(value as i64),
                    )]),
                }
            }
        }
        #[derive(Clone)]
        pub enum StringFilter {
            Equals(String),
            InVec(Vec<String>),
            NotInVec(Vec<String>),
            Lt(String),
            Lte(String),
            Gt(String),
            Gte(String),
            Contains(String),
            StartsWith(String),
            EndsWith(String),
            Not(String),
        }
        impl Into<::prisma_client_rust::SerializedWhereValue> for StringFilter {
            fn into(self) -> ::prisma_client_rust::SerializedWhereValue {
                match self {
                    Self::Equals(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "equals".to_string(),
                            ::prisma_client_rust::PrismaValue::String(value),
                        )])
                    }
                    Self::InVec(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "in".to_string(),
                            ::prisma_client_rust::PrismaValue::List(
                                value
                                    .into_iter()
                                    .map(|value| ::prisma_client_rust::PrismaValue::String(value))
                                    .collect(),
                            ),
                        )])
                    }
                    Self::NotInVec(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "notIn".to_string(),
                            ::prisma_client_rust::PrismaValue::List(
                                value
                                    .into_iter()
                                    .map(|value| ::prisma_client_rust::PrismaValue::String(value))
                                    .collect(),
                            ),
                        )])
                    }
                    Self::Lt(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "lt".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                    Self::Lte(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "lte".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                    Self::Gt(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "gt".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                    Self::Gte(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "gte".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                    Self::Contains(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "contains".to_string(),
                            ::prisma_client_rust::PrismaValue::String(value),
                        )])
                    }
                    Self::StartsWith(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "startsWith".to_string(),
                            ::prisma_client_rust::PrismaValue::String(value),
                        )])
                    }
                    Self::EndsWith(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "endsWith".to_string(),
                            ::prisma_client_rust::PrismaValue::String(value),
                        )])
                    }
                    Self::Not(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "not".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                }
            }
        }
        #[derive(Clone)]
        pub enum StringNullableFilter {
            Equals(Option<String>),
            InVec(Vec<String>),
            NotInVec(Vec<String>),
            Lt(String),
            Lte(String),
            Gt(String),
            Gte(String),
            Contains(String),
            StartsWith(String),
            EndsWith(String),
            Not(Option<String>),
        }
        impl Into<::prisma_client_rust::SerializedWhereValue> for StringNullableFilter {
            fn into(self) -> ::prisma_client_rust::SerializedWhereValue {
                match self {
                    Self::Equals(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "equals".to_string(),
                            value
                                .map(|value| ::prisma_client_rust::PrismaValue::String(value))
                                .unwrap_or_else(|| ::prisma_client_rust::PrismaValue::Null),
                        )])
                    }
                    Self::InVec(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "in".to_string(),
                            ::prisma_client_rust::PrismaValue::List(
                                value
                                    .into_iter()
                                    .map(|value| ::prisma_client_rust::PrismaValue::String(value))
                                    .collect(),
                            ),
                        )])
                    }
                    Self::NotInVec(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "notIn".to_string(),
                            ::prisma_client_rust::PrismaValue::List(
                                value
                                    .into_iter()
                                    .map(|value| ::prisma_client_rust::PrismaValue::String(value))
                                    .collect(),
                            ),
                        )])
                    }
                    Self::Lt(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "lt".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                    Self::Lte(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "lte".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                    Self::Gt(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "gt".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                    Self::Gte(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "gte".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                    Self::Contains(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "contains".to_string(),
                            ::prisma_client_rust::PrismaValue::String(value),
                        )])
                    }
                    Self::StartsWith(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "startsWith".to_string(),
                            ::prisma_client_rust::PrismaValue::String(value),
                        )])
                    }
                    Self::EndsWith(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "endsWith".to_string(),
                            ::prisma_client_rust::PrismaValue::String(value),
                        )])
                    }
                    Self::Not(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "not".to_string(),
                        value
                            .map(|value| ::prisma_client_rust::PrismaValue::String(value))
                            .unwrap_or_else(|| ::prisma_client_rust::PrismaValue::Null),
                    )]),
                }
            }
        }
        #[derive(Clone)]
        pub enum DateTimeFilter {
            Equals(
                ::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>,
            ),
            InVec(
                Vec<
                    ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                >,
            ),
            NotInVec(
                Vec<
                    ::prisma_client_rust::chrono::DateTime<
                        ::prisma_client_rust::chrono::FixedOffset,
                    >,
                >,
            ),
            Lt(::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>),
            Lte(::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>),
            Gt(::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>),
            Gte(::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>),
            Not(::prisma_client_rust::chrono::DateTime<::prisma_client_rust::chrono::FixedOffset>),
        }
        impl Into<::prisma_client_rust::SerializedWhereValue> for DateTimeFilter {
            fn into(self) -> ::prisma_client_rust::SerializedWhereValue {
                match self {
                    Self::Equals(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "equals".to_string(),
                            ::prisma_client_rust::PrismaValue::DateTime(value),
                        )])
                    }
                    Self::InVec(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "in".to_string(),
                            ::prisma_client_rust::PrismaValue::List(
                                value
                                    .into_iter()
                                    .map(|value| ::prisma_client_rust::PrismaValue::DateTime(value))
                                    .collect(),
                            ),
                        )])
                    }
                    Self::NotInVec(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "notIn".to_string(),
                            ::prisma_client_rust::PrismaValue::List(
                                value
                                    .into_iter()
                                    .map(|value| ::prisma_client_rust::PrismaValue::DateTime(value))
                                    .collect(),
                            ),
                        )])
                    }
                    Self::Lt(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "lt".to_string(),
                        ::prisma_client_rust::PrismaValue::DateTime(value),
                    )]),
                    Self::Lte(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "lte".to_string(),
                        ::prisma_client_rust::PrismaValue::DateTime(value),
                    )]),
                    Self::Gt(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "gt".to_string(),
                        ::prisma_client_rust::PrismaValue::DateTime(value),
                    )]),
                    Self::Gte(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "gte".to_string(),
                        ::prisma_client_rust::PrismaValue::DateTime(value),
                    )]),
                    Self::Not(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "not".to_string(),
                        ::prisma_client_rust::PrismaValue::DateTime(value),
                    )]),
                }
            }
        }
    }
}
pub use _prisma::*;
